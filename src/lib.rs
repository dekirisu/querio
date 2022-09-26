pub use querio_derive::*;
pub use strung::prelude::*;
pub use intuple::*;

/* ---------------------------------- Input --------------------------------- */
/// Marks a struct for its fields to be used as query variable inputs.
pub trait QuerioInput {
    fn querio_input(&self) -> String;
}
#[derive(Intuple)]
pub struct QuerioInputUnit;
impl QuerioInput for QuerioInputUnit {
    fn querio_input(&self) -> String {"".to_string()}
}

/* --------------------------------- Output --------------------------------- */
/// Marks a struct as query output.
pub trait QuerioOutput {
    const QUERIO_OUTPUT: &'static str;
}

pub struct QuerioOutputUnit;
impl QuerioOutput for QuerioOutputUnit {
    const QUERIO_OUTPUT: &'static str = "";
}

/* -------------------------------- Variables ------------------------------- */
#[derive(Strung,Intuple)]
pub struct QuerioVariableUnit;
pub type QuerioSectionsUnit = StrungUnit;


/* ---------------------------------- Query --------------------------------- */
/// Marks a struct as query
pub trait Querio {

    /* ---------------------------------- Input --------------------------------- */
    #[cfg(feature = "native_input")]
    type QuerioInputA: QuerioInput + IntupleStruct;
    #[cfg(feature = "native_input")]
    type QuerioInputB: QuerioInput + IntupleStruct;

    /* -------------------------------- Variable -------------------------------- */
    /// Specify varaibles using a struct with the [Strung]-Trait
    #[cfg(feature = "variables")]
    type QuerioVariable: Strung + IntupleStruct;

    /* --------------------------------- Output --------------------------------- */    
    #[cfg(feature = "native_output")]
    type QuerioOutput: QuerioOutput;

    /* ---------------------------------- Query --------------------------------- */
    const QUERY: &'static str;

    /* ------------------------------ Merging Query ----------------------------- */
    /// Merges the query string and the in- and output together to the final query and returns it.
    /// NOTE: Needed 3 inputs for a project I originally made this crate for, might change to 2 at some point
    fn querio(
        #[cfg(feature = "native_input")]
        native_a:&Self::QuerioInputA,
        #[cfg(feature = "native_input")]
        native_b:&Self::QuerioInputB, 
        #[cfg(feature = "variables")]
        hard:&Self::QuerioVariable
    ) -> String {
        let mut query = Self::QUERY.to_string();
        #[cfg(feature = "native_output")]
        {query = query.replace("<Output>",Self::QuerioOutput::QUERIO_OUTPUT);}
        #[cfg(feature = "variables")]
        {query = hard.strung_hashtag(&query);}
        #[cfg(feature = "native_input")]
        {query = query.replace("<Input>",&(native_a.querio_input()+&native_b.querio_input()));}
        query
    }
    /// Shorter form of querio, using tuple as params
    fn qrio(
        #[cfg(feature = "native_input")]
        native_a:<<Self as Querio>::QuerioInputA as IntupleStruct>::Intuple, 
        #[cfg(feature = "native_input")]
        native_b:<<Self as Querio>::QuerioInputB as IntupleStruct>::Intuple, 
        #[cfg(feature = "variables")]
        hard:<<Self as Querio>::QuerioVariable as IntupleStruct>::Intuple, 
    ) -> String { Self::querio(
        #[cfg(feature = "native_input")]
        &Self::QuerioInputA::from_tuple(native_a),
        #[cfg(feature = "native_input")]
        &Self::QuerioInputB::from_tuple(native_b),
        #[cfg(feature = "variables")]
        &Self::QuerioVariable::from_tuple(hard)
    )}

}