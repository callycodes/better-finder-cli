# better-activity-cli

![](https://i.imgur.com/LMg6571.png)

### How to run
#### 1. Build cargo
`cargo build`

#### 2. Setup your activity configs
Using the comments in location_config.rs, setup your acitivty configurations - ideally you'd have one for home/work and split these out per activity.

For example, I personally have 4 configurations setup:
1. Home - for local centres offering Badminton courts
2. Home - for local centres with Squash courts
3. Work - centres in central London with Badminton courts
4. Work - centres in central London with Squash courts

#### 3. Run and find your activity
`cargo run`

Then follow the CLI instructions and enter a date, select an activity config and find a suitable session to book!
