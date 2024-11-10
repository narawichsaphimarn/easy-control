//     pub async fn mouse_control(
//         data_mouse_event: Arc<Mutex<MouseEvent>>,
//         data_protocol_event: Arc<Mutex<ProtocolEvent>>
//     ) -> Result<(), String> {
//         let current_mac = data_protocol_event.lock().unwrap().mac.clone();
//         loop {
//             let _data_protocol_event = data_protocol_event.lock().unwrap();
//             if current_mac != _data_protocol_event.mac {
//                 log::debug!(
//                     "current_mac {} | _data_protocol_event.mac {}",
//                     current_mac,
//                     _data_protocol_event.mac
//                 );
//                 let _data_mouse_event = data_mouse_event.lock().unwrap();
//                 let mouse = mouse_different_pointer(
//                     &(Mouse {
//                         x: _data_mouse_event.x,
//                         y: _data_mouse_event.y,
//                     }),
//                     Screen {
//                         width: _data_protocol_event.source_width,
//                         height: _data_protocol_event.source_height,
//                     },
//                     Screen {
//                         width: _data_protocol_event.target_width,
//                         height: _data_protocol_event.target_height,
//                     }
//                 );
//                 let _ = sent_event(
//                     _data_protocol_event.ip.clone(),
//                     models::mouse_event_model::MouseEvent {
//                         event: 1,
//                         mouse,
//                     }
//                 );
//             }
//         }
//     }
