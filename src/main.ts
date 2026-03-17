import "./styles.css";

async function requestNotificationPermission() {
  if ("Notification" in window) {
    const permission = await Notification.requestPermission();
    console.log("Notification permission:", permission);
  }
}

requestNotificationPermission();
