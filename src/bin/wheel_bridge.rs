use zenoh::Error;
use async_std;

use udp_bridge::wheel_bridge;

#[async_std::main]
async fn main()->Result<(), Error>
{
    let bridge_task = async_std::task::spawn(wheel_bridge("wheel_bridge", "motor/wheel", "192.168.1.50", "192.168.1.50", true));

    bridge_task.await?;

    Ok(())
}