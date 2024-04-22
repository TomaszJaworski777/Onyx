use tch::{
    nn::{self, Module}, Device, Kind, Tensor
};

pub struct SimpleNet {
    pub vs: nn::VarStore,
    net: nn::Sequential,
}

impl SimpleNet {
    pub fn new(architecture: &[usize]) -> SimpleNet {
        let vs: nn::VarStore = nn::VarStore::new(Device::Cpu);
        let mut net = nn::seq();
        let mut iter = architecture.iter();
        let mut input_features = *iter.next().unwrap() as i64;

        for (index, &output_features) in iter.enumerate() {
            net = net.add(nn::linear(
                vs.root() / format!("{index}"),
                input_features,
                output_features as i64,
                Default::default(),
            ));

            if index + 2 < architecture.len() {
                net = net.add_fn(move |xs: &Tensor| xs.clamp(0.0, 1.0).pow_tensor_scalar(2));
            } else {
                net = net.add_fn(move |xs: &Tensor| xs.sigmoid());
            }

            input_features = output_features as i64;
        }

        SimpleNet { vs, net }
    }

    pub fn save(&self, path: &str) -> tch::Result<()> {
        self.vs.save(path)
    }

    pub fn evaluate(&self, input: &Tensor) -> Tensor {
        let output_tensor = self.net.forward(&input).to_kind(Kind::Float);
        output_tensor
    }
}
