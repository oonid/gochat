pub fn get_notification_script() -> String {
    r#"
(function() {
    if (window.__gochatNotificationMonitor) return;
    window.__gochatNotificationMonitor = true;

    function requestNotificationPermission() {
        if ('Notification' in window) {
            Notification.requestPermission().then(function(permission) {
                console.log('GoChat: Notification permission:', permission);
                if (permission === 'denied') {
                    if (window.__TAURI__ && window.__TAURI__.event) {
                        window.__TAURI__.event.emit('notification-permission', { status: 'denied' });
                    }
                }
            });
        }
    }

    function interceptNotifications() {
        const OriginalNotification = window.Notification;

        window.Notification = function(title, options) {
            if (window.__TAURI__ && window.__TAURI__.event) {
                window.__TAURI__.event.emit('desktop-notification', {
                    title: title || '',
                    body: options?.body || '',
                    icon: options?.icon || '',
                    tag: options?.tag || ''
                });
            }

            if (OriginalNotification.permission === 'granted') {
                return new OriginalNotification(title, options);
            }
            return null;
        };

        window.Notification.permission = OriginalNotification.permission;
        window.Notification.requestPermission = OriginalNotification.requestPermission.bind(OriginalNotification);
    }

    function monitorDocumentTitle() {
        let lastTitle = document.title;
        
        const titleObserver = new MutationObserver(function() {
            if (document.title !== lastTitle) {
                const oldTitle = lastTitle;
                lastTitle = document.title;
                
                if (document.title.includes('(') && document.title.includes(')')) {
                    const match = document.title.match(/\((\d+)\)/);
                    if (match) {
                        if (window.__TAURI__ && window.__TAURI__.event) {
                            window.__TAURI__.event.emit('unread-count', { count: parseInt(match[1], 10) });
                        }
                    }
                }
            }
        });

        titleObserver.observe(document.querySelector('title') || document.documentElement, {
            childList: true,
            characterData: true,
            subtree: true
        });
    }

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', function() {
            requestNotificationPermission();
            interceptNotifications();
            monitorDocumentTitle();
        });
    } else {
        requestNotificationPermission();
        interceptNotifications();
        monitorDocumentTitle();
    }

    console.log('GoChat: Notification monitor initialized');
})();
"#.to_string()
}

pub fn get_favicon_monitor_script() -> String {
    r#"
(function() {
    if (window.__gochatFaviconMonitor) return;
    window.__gochatFaviconMonitor = true;

    const FAVICON_SELECTORS = [
        'link#favicon256',
        'link[rel="shortcut icon"]',
        'link[rel="icon"]',
        'link[type="image/x-icon"]'
    ];

    let lastHref = null;
    let lastState = 'unknown';
    let pollInterval = null;

    function getFaviconHref() {
        for (const selector of FAVICON_SELECTORS) {
            const el = document.querySelector(selector);
            if (el && el.href) {
                return el.href;
            }
        }
        return null;
    }

    function determineState(href) {
        if (!href) return 'offline';
        
        const hrefLower = href.toLowerCase();
        
        if (hrefLower.includes('offline') || 
            hrefLower.includes('disconnected') ||
            hrefLower.includes('error')) {
            return 'offline';
        }
        
        if (hrefLower.includes('badge') || 
            hrefLower.includes('unread') ||
            hrefLower.includes('notification') ||
            hrefLower.includes('alert') ||
            hrefLower.includes('chat') && hrefLower.includes('new')) {
            return 'badge';
        }
        
        if (hrefLower.includes('chat') || 
            hrefLower.includes('default') ||
            hrefLower.includes('favicon')) {
            return 'normal';
        }
        
        return 'normal';
    }

    function emitState(state) {
        if (state !== lastState) {
            lastState = state;
            try {
                if (window.__TAURI__ && window.__TAURI__.event) {
                    window.__TAURI__.event.emit('favicon-changed', { state: state, href: lastHref });
                }
            } catch (e) {
                console.warn('GoChat: Failed to emit favicon event:', e);
            }
        }
    }

    function checkFavicon() {
        try {
            const href = getFaviconHref();
            if (href !== lastHref) {
                lastHref = href;
                const state = determineState(href);
                emitState(state);
            }
        } catch (e) {
            console.warn('GoChat: Favicon check error:', e);
            emitState('offline');
        }
    }

    function startMonitoring() {
        if (pollInterval) return;
        
        pollInterval = setInterval(checkFavicon, 1500);
        
        checkFavicon();
    }

    function stopMonitoring() {
        if (pollInterval) {
            clearInterval(pollInterval);
            pollInterval = null;
        }
    }

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', startMonitoring);
    } else {
        startMonitoring();
    }

    window.addEventListener('beforeunload', stopMonitoring);

    const observer = new MutationObserver(function(mutations) {
        for (const mutation of mutations) {
            for (const node of mutation.addedNodes) {
                if (node.nodeName === 'LINK' && 
                    (node.rel === 'icon' || node.rel === 'shortcut icon')) {
                    checkFavicon();
                    return;
                }
            }
        }
    });

    observer.observe(document.head || document.documentElement, {
        childList: true,
        subtree: true
    });

    console.log('GoChat: Favicon monitor initialized');
})();
"#.to_string()
}

pub fn get_css_injection_script(css: &str) -> String {
    let escaped_css = css
        .replace('\\', "\\\\")
        .replace('`', "\\`")
        .replace('$', "\\$");

    format!(
        r#"
(function() {{
    if (window.__gochatCustomCSS) return;
    window.__gochatCustomCSS = true;
    
    try {{
        const style = document.createElement('style');
        style.id = 'gochat-custom-css';
        style.textContent = `{}`;
        
        if (document.head) {{
            document.head.appendChild(style);
        }} else {{
            document.addEventListener('DOMContentLoaded', function() {{
                document.head.appendChild(style);
            }});
        }}
        
        console.log('GoChat: Custom CSS injected successfully');
    }} catch (e) {{
        console.error('GoChat: Failed to inject custom CSS:', e);
    }}
}})();
"#,
        escaped_css
    )
}
