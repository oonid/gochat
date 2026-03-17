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
