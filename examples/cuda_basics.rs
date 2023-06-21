use anyhow::Result;
use candle::{Device, Tensor};

fn main() -> Result<()> {
    let device = Device::new_cuda(0)?;
    let x = Tensor::new(&[3f32, 1., 4., 1., 5.], &device)?;
    println!("{:?}", x.to_vec1::<f32>()?);
    let y = Tensor::new(&[2f32, 7., 1., 8., 2.], &device)?;
    let z = (y * 3.)?;
    println!("{:?}", z.to_vec1::<f32>()?);
    Ok(())
}
