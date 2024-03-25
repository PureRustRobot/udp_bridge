use zenoh::Error;
use async_std;

use udp_bridge::one_motor_bridge;

#[async_std::main]
async fn main()->Result<(), Error>
{
    let bridge_task = async_std::task::spawn(one_motor_bridge("motor_bridge", "motor/updown", "192.168.1.50", "192.168.1.50", true));

    bridge_task.await?;

    Ok(())
}