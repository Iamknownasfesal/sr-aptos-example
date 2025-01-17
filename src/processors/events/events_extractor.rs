use crate::db::common::models::events_models::EventModel;
use anyhow::Result;
use aptos_indexer_processor_sdk::{
    aptos_protos::transaction::v1::{transaction::TxnData, Transaction},
    traits::{async_step::AsyncRunType, AsyncStep, NamedStep, Processable},
    types::transaction_context::TransactionContext,
    utils::errors::ProcessorError,
};
use async_trait::async_trait;
use strum::IntoEnumIterator;
use rayon::prelude::*;
use tracing::warn;

use crate::processors::events::events_sr_amm::Events;

/// EventsExtractor is a step that extracts events and their metadata from transactions.
pub struct EventsExtractor
where
    Self: Sized + Send + 'static, {}

#[async_trait]
impl Processable for EventsExtractor {
    type Input = Vec<Transaction>;
    type Output = Vec<EventModel>;
    type RunType = AsyncRunType;

    async fn process(
        &mut self,
        item: TransactionContext<Vec<Transaction>>,
    ) -> Result<Option<TransactionContext<Vec<EventModel>>>, ProcessorError> {
        let events = item
            .data
            .par_iter()
            .map(|txn| {
                let txn_version = txn.version as i64;
                let block_height = txn.block_height as i64;
                let txn_data = match txn.txn_data.as_ref() {
                    Some(data) => data,
                    None => {
                        warn!(
                            transaction_version = txn_version,
                            "Transaction data doesn't exist"
                        );
                        return vec![];
                    }
                };

                let raw_events = match txn_data {
                    TxnData::User(tx_inner) => &tx_inner.events,
                    _ => &vec![],
                };

                let filtered_events: Vec<_> = raw_events
                    .iter()
                    .filter(|e| Events::iter().any(|event| e.type_str == event.as_str()))
                    .cloned()
                    .collect();

                EventModel::from_events(&filtered_events, txn_version, block_height)
            })
            .flatten()
            .collect::<Vec<EventModel>>();
        Ok(Some(TransactionContext {
            data: events,
            metadata: item.metadata,
        }))
    }
}

impl AsyncStep for EventsExtractor {}

impl NamedStep for EventsExtractor {
    fn name(&self) -> String {
        "EventsExtractor".to_string()
    }
}
