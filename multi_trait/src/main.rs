use multi_trait::Model;

fn main() {
    let model = Model::default();
    {
        use multi_trait::a::Test;
        model.test();
    }
    {
        use multi_trait::b::Test;
        model.test();
    }
}
