// use aws_iot_device_sdk_rust::{async_event_loop_listener, AWSIoTAsyncClient, AWSIoTSettings};
// use rumqttc::{self, Packet, QoS};
// use serde::{Serialize, Deserialize};
// use std::error::Error;

// #[derive(Serialize, Deserialize)]
// struct Message {
//     text: String,
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {

//     let aws_settings = AWSIoTSettings::new(
//         "Iot-boschecl2".to_owned(),
//         "/home/santosh/rust development/project_with_yashwanth/aws-iot-device-sdk-rust/AmazonRootCA1.pem".to_owned(),
//         "/home/santosh/rust development/project_with_yashwanth/aws-iot-device-sdk-rust/38912c2dc782b6f9e7f9e7f6cc7c1b0beb50a875f4fc171d7f4eb47901049d3a-certificate.pem.crt".to_owned(),
//         "/home/santosh/rust development/project_with_yashwanth/aws-iot-device-sdk-rust/38912c2dc782b6f9e7f9e7f6cc7c1b0beb50a875f4fc171d7f4eb47901049d3a-private.pem.key".to_owned(),
//         "a35lo5fxmqnlto-ats.iot.us-east-1.amazonaws.com".to_owned(),
//         None,
//     );

//     let (iot_core_client, eventloop_stuff) = AWSIoTAsyncClient::new(aws_settings).await?;

//     iot_core_client.subscribe("test1234".to_string(), QoS::AtMostOnce).await.unwrap();
//     iot_core_client.publish("test1234".to_string(), QoS::AtMostOnce, "hey").await.unwrap();

//     let mut receiver1 = iot_core_client.get_receiver().await;
//     let mut receiver2 = iot_core_client.get_receiver().await;

//     let recv1_thread = tokio::spawn(async move {
//         loop {
//             match receiver1.recv().await {
//                 Ok(event) =>{
//                     match event{
//                     Packet::Publish(p) => {
//                         println!("Received message {:?} on topic: {}", p.payload, p.topic)
//                     }
//                     _ => println!("Got event on receiver1: {:?}", event),
//                 }
//                 },
//                 Err(_)=>(),
//             }

//         }
//     });

//     let recv2_thread = tokio::spawn(async move {
//         loop {
//             match receiver2.recv().await{
//                 Ok(event)=> println!("Got event on receiver2 : {:?}", event),
//                 Err(_)=>(),
//             }
//         }
//     });


//     let publish = tokio::spawn(async move {
//         loop {
//             iot_core_client.publish("test1234".to_string(), QoS::AtMostOnce, "hello santosh").await.unwrap();
//         }
//     });
//     let listen_thread = tokio::spawn(async move {
//         async_event_loop_listener(eventloop_stuff).await.unwrap();
//         //iot_core_client.listen().await.unwrap();
//     });

//     //iot_core_client.publish("topic".to_string(), QoS::AtMostOnce, "hey").await.unwrap();
//     // tokio::join!
//     //     (recv1_thread,
//     //     recv2_thread,
//     //     listen_thread,publish);

//         tokio::join!(publish);

//     Ok(())
// }






use aws_iot_device_sdk_rust::{async_event_loop_listener, AWSIoTAsyncClient, AWSIoTSettings};
use rumqttc::{self, Packet, QoS};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct Message {
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let aws_settings = AWSIoTSettings::new(
        "Iot-boschecl2".to_owned(),
        "/home/santosh/rust development/project_with_yashwanth/aws-iot-device-sdk-rust/AmazonRootCA1.pem".to_owned(),
        "/home/santosh/rust development/project_with_yashwanth/aws-iot-device-sdk-rust/38912c2dc782b6f9e7f9e7f6cc7c1b0beb50a875f4fc171d7f4eb47901049d3a-certificate.pem.crt".to_owned(),
        "/home/santosh/rust development/project_with_yashwanth/aws-iot-device-sdk-rust/38912c2dc782b6f9e7f9e7f6cc7c1b0beb50a875f4fc171d7f4eb47901049d3a-private.pem.key".to_owned(),
        "a35lo5fxmqnlto-ats.iot.us-east-1.amazonaws.com".to_owned(),
        None,
    );

    let (iot_core_client, eventloop_stuff) = AWSIoTAsyncClient::new(aws_settings).await?;

    iot_core_client.subscribe("test1234".to_string(), QoS::AtMostOnce).await.unwrap();
    iot_core_client.publish("test1234".to_string(), QoS::AtMostOnce, "hey").await.unwrap();

    let mut receiver1 = iot_core_client.get_receiver().await;
    let mut receiver2 = iot_core_client.get_receiver().await;

    // let recv1_thread = tokio::spawn(async move {
    //     loop {
    //         match receiver1.recv().await {
    //             Ok(event) => {
    //                 match event {
    //                     Packet::Publish(p) => {
    //                         println!("Received message {:?} on topic: {}", p.payload, p.topic)
    //                     }
    //                     _ => println!("Got event on receiver1: {:?}", event),
    //                 }
    //             }
    //             Err(_) => (),
    //         }
    //     }
    // });

    // let recv2_thread = tokio::spawn(async move {
    //     loop {
    //         match receiver2.recv().await {
    //             Ok(event) => println!("Got event on receiver2 : {:?}", event),
    //             Err(_) => (),
    //         }
    //     }
    // });

    let publish = tokio::spawn(async move {
        loop {
            let message = Message {
                text: "hello santosh".to_string(),
            };
            let json_message = serde_json::to_string(&message).unwrap();
            iot_core_client.publish("test1234".to_string(), QoS::AtMostOnce, &*json_message).await.unwrap();
        }
    });
    
    let listen_thread = tokio::spawn(async move {
        async_event_loop_listener(eventloop_stuff).await.unwrap();
    });

    tokio::join!(
        // recv1_thread, recv2_thread, listen_thread,
         publish);

    Ok(())
}
