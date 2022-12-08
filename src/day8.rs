type Tree = u8;

#[derive(Debug)]
struct Forest {
    trees: Vec<Vec<Tree>>,
    rows: usize,
    columns: usize,
}

impl Forest {
    pub fn new(input: &str) -> Forest {
        let mut forest = Forest {
            trees: Vec::new(),
            rows: 0,
            columns: 0,
        };

        for line in input.lines() {
            let mut row: Vec<Tree> = Vec::new();

            for c in line.chars() {
                let tree = c.to_digit(10).expect("Every tree should be a digit") as Tree;
                row.push(tree);
            }

            forest.trees.push(row);
        }

        forest.rows = forest.trees.len();
        forest.columns = forest.trees[0].len();

        return forest;
    }

    fn is_visible(&self, row_index: usize, tree_index: usize) -> bool {
        if row_index == 0
            || tree_index == 0
            || row_index == self.rows - 1
            || tree_index == self.columns - 1
        {
            return true;
        }

        let tree = self.trees[row_index][tree_index];

        let mut visible_from_left = true;
        let mut visible_from_right = true;
        let mut visible_from_top = true;
        let mut visible_from_bottom = true;

        for row_tree in &self.trees[row_index][0..tree_index] {
            if row_tree >= &tree {
                visible_from_left = false;
            }
        }
        for row_tree in &self.trees[row_index][tree_index + 1..] {
            if row_tree >= &tree {
                visible_from_right = false;
            }
        }

        for row in &self.trees[0..row_index] {
            if row[tree_index] >= tree {
                visible_from_top = false;
            }
        }
        for row in &self.trees[row_index + 1..] {
            if row[tree_index] >= tree {
                visible_from_bottom = false;
            }
        }

        return visible_from_bottom || visible_from_top || visible_from_left || visible_from_right;
    }

    fn scenic_score(&self, row_index: usize, tree_index: usize) -> u64 {
        let mut trees_on_left = 0;
        let mut trees_on_right = 0;
        let mut trees_on_top = 0;
        let mut trees_on_bottom = 0;

        let tree = self.trees[row_index][tree_index];

        if row_index == 0
            || tree_index == 0
            || row_index == self.rows - 1
            || tree_index == self.columns - 1
        {
            return 0;
        }

        for row_tree in self.trees[row_index][0..tree_index].iter().rev() {
            if row_tree < &tree {
                trees_on_left += 1;
            } else {
                trees_on_left += 1;
                break;
            }
        }

        for row_tree in &self.trees[row_index][tree_index + 1..] {
            if row_tree < &tree {
                trees_on_right += 1;
            } else {
                trees_on_right += 1;
                break;
            }
        }

        for row in self.trees[0..row_index].iter().rev() {
            if row[tree_index] < tree {
                trees_on_top += 1;
            } else {
                trees_on_top += 1;
                break;
            }
        }

        for row in &self.trees[row_index + 1..] {
            if row[tree_index] < tree {
                trees_on_bottom += 1;
            } else {
                trees_on_bottom += 1;
                break;
            }
        }

        return trees_on_left * trees_on_right * trees_on_top * trees_on_bottom;
    }
}

pub fn run() {
    let input = include_str!("../inputs/day8.txt");
    let forest = Forest::new(input);

    let mut visible = 0;

    for (row_index, row) in forest.trees.iter().enumerate() {
        for (tree_index, _) in row.iter().enumerate() {
            if forest.is_visible(row_index, tree_index) {
                visible += 1;
            };
        }
    }

    println!("Visible trees: {}", visible);

    let mut max_scenic_score = 0;

    for (row_index, row) in forest.trees.iter().enumerate() {
        for (tree_index, _) in row.iter().enumerate() {
            let scenic_score = forest.scenic_score(row_index, tree_index);
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    println!("Max scenic score: {}", max_scenic_score);
}
