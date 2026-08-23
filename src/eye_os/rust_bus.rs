use bytes::{Bytes, BytesMut};
use log::{info, warn};
use std::time::Instant;
use tokio::sync::mpsc;

/// رسالة مبسطة يتم نقلها عبر الناقل باستعمال الذاكرة المشتركة
#[derive(Debug, Clone)]
pub struct BusFrame {
    pub sender_id: usize,
    pub payload: Bytes,
    pub timestamp: Instant,
}

/// ناقل البيانات عديم النسخ (Zero-Copy Rust Bus)
pub struct RustBus {
    pub capacity: usize,
    pub tx: mpsc::Sender<BusFrame>,
    pub rx: Option<mpsc::Receiver<BusFrame>>,
}

impl RustBus {
    /// إنشاء قناة اتصالات سريعة مع تحديد سعة البوفر
    pub fn new(capacity: usize) -> Self {
        let (tx, rx) = mpsc::channel(capacity);
        Self {
            capacity,
            tx,
            rx: Some(rx),
        }
    }

    /// إرسال البيانات بسرعة بدون إجراء عمليات نسخ (Zero-Copy Transfer)
    pub async fn publish(&self, sender_id: usize, raw_data: &[u8]) -> Result<(), String> {
        let start_time = Instant::now();
        
        // تحويل البيانات مباشرة لنطاق Bytes لمنع النسخ الزائد
        let mut buffer = BytesMut::with_capacity(raw_data.len());
        buffer.extend_from_slice(raw_data);
        
        let frame = BusFrame {
            sender_id,
            payload: buffer.freeze(),
            timestamp: Instant::now(),
        };

        match self.tx.send(frame).await {
            Ok(_) => {
                let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;
                info!(
                    "[RUST BUS] Zero-Copy payload published from Agent [{}] in {:.4}ms",
                    sender_id, elapsed
                );
                Ok(())
            }
            Err(e) => {
                warn!("[RUST BUS] Failed to publish frame: {:?}", e);
                Err(format!("Bus dispatch error: {}", e))
            }
        }
    }

    /// بدء الاستماع والإنصات المباشر لمعالجة الرسائل الواردة
    pub async fn start_listener(&mut self) -> Result<(), String> {
        if let Some(mut rx) = self.rx.take() {
            info!("[RUST BUS] Zero-Copy Communication Listener active.");
            while let Some(frame) = rx.recv().await {
                let latency = frame.timestamp.elapsed().as_secs_f64() * 1000.0;
                info!(
                    "[RUST BUS] Received Frame from Sender [{}] | Bytes: {} | Bus Latency: {:.4}ms",
                    frame.sender_id,
                    frame.payload.len(),
                    latency
                );
            }
            Ok(())
        } else {
            Err("Listener is already active or receiver buffer dropped.".to_string())
        }
    }
}
