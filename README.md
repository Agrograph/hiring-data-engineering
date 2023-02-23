# hiring-data-engineering

Data Engineering Rust programming exercise

### Overview

The goal of this exercise is to gain insight into how you, as an engineer, work to solve problems. These problems may consist of understanding the Rust language, what data structures to use, and what the most optimal solution is. The goal is not to 'complete' the exercise, as much as it is to demonstrate your though pattern.

### Project Structure

- `/files`
  - `/people.csv` - input data source for `person`
  - `/scores.csv`- input data source for `score`
- `/out`
  - `/_example_01.json` - example JSON output
  - `/_example_02.csv` - example CSV output
- `/src`
  - `/examples.rs` - a collection of JSON & CSV example functions
  - `/main.rs` - main file

### Exercises

#### 1. Most common professions

##### Context

A client wants to process the `people.csv` to generate a report on the most common professions.

##### Tasks

- [ ] Generate a `.csv` file to display the most common professions
- [ ] The CSV should contain the `profession` and `count` columns
- [ ] The CSV should be sorted from most common to least common

#### 2. Highest scoring people

##### Context

A client wants to process `people.csv` and `scores.csv` to generate a report on the highest scores. The `id` column on `people.csv` should map to the `id` column on `scores.csv`.

##### Tasks

- [ ] Generate a `.csv` file to display the highest scoring people
- [ ] The CSV should contain the `id`, `firstname`, `lastname`, and `score` columns
- [ ] The CSV should be sorted from highest scoring to lowest scoring

#### 3. Aggregate profession score

##### Context

A client wants to process `people.csv` and `scores.csv` to generate a report on the sum of all scores grouped by profession. The `id` column on `people.csv` should map to the `id` column on `scores.csv`.

##### Tasks

- [ ] Generate a `.json` file to display the score sum of each profession
- [ ] The JSON file should contain properties `profession` which map to `score`

example:

```json
{
	"doctor": 533,
	"developer": 239
}
```
