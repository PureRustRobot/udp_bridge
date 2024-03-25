use zenoh::{
    config::Config, prelude::r#async::*,
    Error
};

use prr_msgs::msg::*;
use prr_utils::logger;

use async_std::net::UdpSocket;

pub async fn wheel_bridge(
    name:&str,
    sub_topic:&str,
    local_addr:&str,
    remote_addr:&str,
    enable_debug:bool
)->Result<(), Error>
{
    let session = zenoh::open(Config::default()).res().await.unwrap();

    let subscriber = session.declare_subscriber(sub_topic).res().await.unwrap();

    let sock = UdpSocket::bind(local_addr).await.unwrap();

    logger::log_info(name, format!("Start {} sub_topic:{}, local:{}, remote:{}", name, sub_topic, local_addr, remote_addr));

    loop {
        let sub = subscriber.recv_async().await.unwrap();

        let get_data = Wheel::deserialize(sub.value.to_string());

        match sock.send_to(Wheel::serialize(&get_data).as_bytes(), remote_addr).await {
            Ok(_)=>{
                if enable_debug
                {                
                    logger::log_info(name, format!("Send fl:{}, fr:{}, rl:{}, rr:{}", get_data.front_left, get_data.front_right, get_data.rear_left, get_data.rear_right));
                }
            }
            Err(err)=>{
                logger::log_error(name, format!("Failed to send to {}:{}", remote_addr, err));
            }
        }
    }
}

pub async fn one_motor_bridge(
    name:&str,
    sub_topic:&str,
    local_addr:&str,
    remote_addr:&str,
    enable_debug:bool
)->Result<(), Error>
{
    let session = zenoh::open(Config::default()).res().await.unwrap();

    let subscriber = session.declare_subscriber(sub_topic).res().await.unwrap();

    let sock = UdpSocket::bind(local_addr).await.unwrap();

    logger::log_info(name, format!("Start {} sub_topic:{}, local:{}, remote:{}", name, sub_topic, local_addr, remote_addr));

    loop {
        let sub = subscriber.recv_async().await.unwrap();

        let get_data = Motor::deserialize(sub.value.to_string());

        match sock.send_to(Motor::serialize(&get_data).as_bytes(), remote_addr).await {
            Ok(_)=>{
                if enable_debug
                {                
                    logger::log_info(name, format!("Send motor_power:{}", get_data.power));
                }
            }
            Err(err)=>{
                logger::log_error(name, format!("Failed to send to {}:{}", remote_addr, err));
            }
        }
    }
}