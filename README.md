# F1 LED Circuit Timestamp Consolidator

This Rust program processes an input CSV file containing timestamped data from the 2023 Formula 1 race at the Zandvoort Circuit. IT consolidates the timestamps to the nearest anchor points set at regular intervals (100 ms). The processed data is then saved to an output CSV file with the required format.

## Features

- Reads input data from a CSV file.
- Parses timestamps and sets anchor points every 100 milliseconds.
- Adjusts timestamps in the data to the nearest anchor points.
- Formats the designator field by removing the prefix 'U' and renaming it to 'led_num'.
- Outputs the processed data to a new CSV file with only the necessary fields: `timestamp`, `led_num`, and `driver_number`.
