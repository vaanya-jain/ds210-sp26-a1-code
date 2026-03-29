use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};

pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    todo!("Implement this!");
}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    todo!("Implement this!");
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    let mut result = HashMap::new();
    for (group_value, group_dataset) in dataset {
        let aggregated = match aggregation {
            Aggregation::Count(_) => {
                Value::Integer(group_dataset.len() as i32)
            }

            Aggregation::Sum(column_name) => {
                let idx = group_dataset.column_index(column_name);
                let mut sum = 0;

                for row in group_dataset.iter() {
                    if let Value::Integer(num) = row.get_value(idx) {
                        sum += *num;
                    }
                }

                Value::Integer(sum)
            }

            Aggregation::Average(column_name) => {
                let idx = group_dataset.column_index(column_name);
                let mut sum = 0;
                let mut count = 0;

                for row in group_dataset.iter() {
                    if let Value::Integer(num) = row.get_value(idx) {
                        sum += *num;
                        count += 1;
                    }
                }

                if count == 0 {
                    Value::Integer(0)
                } else {
                    Value::Integer(sum / count)
                }
            }
        };

        result.insert(group_value, aggregated);
    }

    result


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
        (query.get_aggregate().get_result_column_name(), ColumnType::Integer),
    ];

    // Create result dataset object and fill it with the results.
    let mut result = Dataset::new(columns);
    for (grouped_value, aggregation_value) in aggregated {
        result.add_row(Row::new(vec![grouped_value, aggregation_value]));
    }
    return result;
}