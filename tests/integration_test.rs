use rsalgo;

#[test]
fn base() {
    let mut gen = rsalgo::base::SubsetGenerator::new(1,1);
    assert_eq!(Some(0),gen.next());
}