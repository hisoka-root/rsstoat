use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread::{self, JoinHandle};
use std::time::Duration;

const DISCORD_CLIENT_ID: &str = "872068124005007420";
const RECONNECT_DELAY: Duration = Duration::from_secs(10);

pub struct DiscordRpc {
    running: Arc<AtomicBool>,
    handle: Mutex<Option<JoinHandle<()>>>,
}

impl DiscordRpc {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            handle: Mutex::new(None),
        }
    }

    pub fn start(&self) {
        self.stop();
        self.running.store(true, Ordering::SeqCst);
        let running = self.running.clone();
        let handle = thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                if let Err(()) = run_session(&running) {
                    if !running.load(Ordering::SeqCst) {
                        break;
                    }
                    thread::sleep(RECONNECT_DELAY);
                } else {
                    break;
                }
            }
        });
        *self.handle.lock().unwrap() = Some(handle);
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.handle.lock().unwrap().take() {
            let _ = handle.join();
        }
    }
}

fn run_session(running: &AtomicBool) -> Result<(), ()> {
    let mut client = DiscordIpcClient::new(DISCORD_CLIENT_ID);
    client.connect().map_err(|_| ())?;

    let activity = discord_rich_presence::activity::Activity::new()
        .state("stoat.chat")
        .details("Chatting with others")
        .assets(
            discord_rich_presence::activity::Assets::new()
                .large_image("qr")
                .large_text("Join Stoat!"),
        )
        .buttons(vec![discord_rich_presence::activity::Button::new(
            "Join Stoat",
            "https://stoat.chat/",
        )]);

    let _ = client.set_activity(activity);

    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_secs(30));
        let activity = discord_rich_presence::activity::Activity::new()
            .state("stoat.chat")
            .details("Chatting with others")
            .assets(
                discord_rich_presence::activity::Assets::new()
                    .large_image("qr")
                    .large_text("Join Stoat!"),
            )
            .buttons(vec![discord_rich_presence::activity::Button::new(
                "Join Stoat",
                "https://stoat.chat/",
            )]);
        if client.set_activity(activity).is_err() {
            return Err(());
        }
    }

    Ok(())
}
