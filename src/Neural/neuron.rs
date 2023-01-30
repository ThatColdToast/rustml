pub struct Neuron {
    pub value: f32,
    lastLayerWeights: Vec<f32>,
    bias: f32
}

impl Neuron {
    fn initRandWide() -> f32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        return 2.0 * rng.gen::<f32>() - 1.0;
    }

    fn initRandNarrow() -> f32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        return rng.gen::<f32>();
    }

    pub fn new(lastLayerSize: i32) -> Neuron {

        let value: f32 = Self::initRandNarrow();

        // println!("NVal: {}", value);

        let mut tempWeights: Vec<f32> = Vec::new();

        for i in 0..lastLayerSize {
            tempWeights.push(Self::initRandWide());
        }

        return Neuron {
            value: value,
            lastLayerWeights: tempWeights,
            bias: Self::initRandWide()
        }
    }

    fn activate(val: f32) -> f32 {
        use activation_functions::f32;
        return f32::sigmoid(val);
    }

    pub fn propagate(&mut self, lastLayerNeurons: &Vec<Neuron>) {
        if lastLayerNeurons.len() != self.lastLayerWeights.len() {
            panic!("Layer Sizes Changed!")
        }

        for i in 0..lastLayerNeurons.len() {
            // println!("Weight Prop: #{} - V:{} - W:{}", i, lastLayerNeurons[i].value, self.lastLayerWeights[i]);
            self.value += lastLayerNeurons[i].value * self.lastLayerWeights[i]
        }

        self.value += self.bias;

        self.value = Self::activate(self.value);

        // println!("Neuron Propagate Value: {}", self.value);
    }

    pub fn print(&self) {
        print!("{:.2} ", self.value);
    }
}