use communication::*;
use communication::pact::Pipeline;

use example_shared::*;
use example_shared::operators::unary::UnaryStreamExt;

pub trait InspectExt<D: Data> {
    fn inspect<F: FnMut(&D)+'static>(&self, func: F) -> Self;
}

impl<G: GraphBuilder, D: Data> InspectExt<D> for Stream<G, D> {
    fn inspect<F: FnMut(&D)+'static>(&self, mut func: F) -> Stream<G, D> {
        self.unary_stream(Pipeline, format!("Inspect"), move |input, output| {
            while let Some((time, data)) = input.pull() {
                for datum in &*data { func(datum); }
                output.give_vector_at(&time, data);
            }
        })
    }
}


pub trait InspectBatchExt<G: GraphBuilder, D: Data> {
    fn inspect_batch<F: FnMut(&G::Timestamp, &Vec<D>)+'static>(&self, mut func: F) -> Self;
}

impl<G: GraphBuilder, D: Data> InspectBatchExt<G, D> for Stream<G, D> {
    fn inspect_batch<F: FnMut(&G::Timestamp, &Vec<D>)+'static>(&self, mut func: F) -> Stream<G, D> {
        self.unary_stream(Pipeline, format!("Inspect"), move |input, output| {
            while let Some((time, data)) = input.pull() {
                func(&time, data);
                output.give_vector_at(&time, data);
            }
        })
    }
}
