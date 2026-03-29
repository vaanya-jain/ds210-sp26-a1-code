use crate::dataset::{ColumnType, Dataset, Row, Value};
use crate::query::{Aggregation, Condition, Query};
use std::collections::HashMap;

pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    todo!("Implement this!");
}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    let mut grouped = HashMap::new();

    let columns = dataset.columns().clone();
    let group_index = dataset.column_index(group_by_column);

    for row in dataset.into_iter() {
        let group_value = row.get_value(group_index).clone();

        if !grouped.contains_key(&group_value) {
            grouped.insert(group_value.clone(), Dataset::new(columns.clone()));
        }

        grouped.get_mut(&group_value).unwrap().add_row(row);
    }

    return grouped;
}

pub fn aggregate_dataset(
    dataset: HashMap<Value, Dataset>,
    aggregation: &Aggregation,
) -> HashMap<Value, Value> {
    todo!("Implement this!");
}

pub fn compute_query_on_dataset(dataset: &Dataset, query: &Query) -> Dataset {
    let filtered = filter_dataset(dataset, query.get_filter());
    let grouped = group_by_dataset(filtered, query.get_group_by());
    let aggregated = aggregate_dataset(grouped, query.get_aggregate());

    // Create the name of the columns.
    let group_by_column_name = query.get_group_by();
    let group_by_column_type = dataset.column_type(group_by_column_name);
    let columns = vec![
        (group_by_column_name.clone(), group_by_column_type.clone()),
        (
            query.get_aggregate().get_result_column_name(),
            ColumnType::Integer,
        ),
    ];

    // Create result dataset object and fill it with the results.
    let mut result = Dataset::new(columns);
    for (grouped_value, aggregation_value) in aggregated {
        result.add_row(Row::new(vec![grouped_value, aggregation_value]));
    }
    return result;
}
