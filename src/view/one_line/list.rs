use crate::data::activity;
use crate::data::processor::ListData;
use crate::data::processor::ListWriter;
use crate::view::format_util::format_duration;

pub struct Writer {}

impl ListWriter for Writer {
    fn process(&self, data: &ListData) -> anyhow::Result<()> {
        list_activities(&&data.activities)
    }
}

pub fn list_activities(activities: &[&activity::Activity]) -> anyhow::Result<()> {
    if activities.is_empty() {
        return Err(anyhow::anyhow!("No activity to display"));
    }
    if activities.len() > 1 {
        return Err(anyhow::anyhow!("Several activities not expected"));
    }

    let activity = activities[0];
    print_row(activity);
    Ok(())
}

fn print_row(activity: &activity::Activity) {
    print!("{} ({})", activity.description, format_duration(&activity.get_duration()) )
}
