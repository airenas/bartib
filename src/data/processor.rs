use anyhow::Result;
use chrono::Duration;

use crate::data::activity;
use crate::data::round_util::round_datetime;

pub type ProcessorList = Vec<Box<dyn ActivityProcessor>>;

pub trait ActivityProcessor {
    fn process(&self, activity: &activity::Activity) -> activity::Activity;
}

pub struct StatusReportData<'a> {
    pub activity: Option<&'a activity::Activity>,
    pub project: Option<&'a str>,
    pub today: Duration,
    pub current_week: Duration,
    pub current_month: Duration,
}
pub trait StatusReportWriter {
    fn process(&self, data: &StatusReportData) -> Result<()>;
}

pub struct ListData<'a> {
    pub activities: Vec<&'a activity::Activity>,
    pub do_group_activities: bool,
    pub with_start_dates: bool,
}

pub trait ListWriter {
    fn process(&self, data: &ListData) -> Result<()>;
}

pub struct ReportData<'a> {
    pub activities: Vec<&'a activity::Activity>,
}

pub trait ReportWriter {
    fn process(&self, data: &ReportData) -> Result<()>;
}

pub struct RoundProcessor {
    pub round: Duration,
}

pub struct ShiftProcessor {
    pub shift: Duration,
}

impl ActivityProcessor for RoundProcessor {
    fn process(&self, activity: &activity::Activity) -> activity::Activity {
        let start = round_datetime(&activity.start, &self.round);
        let end = activity.end.map(|end| round_datetime(&end, &self.round));

        activity::Activity {
            start,
            end,
            project: activity.project.clone(),
            description: activity.description.clone(),
        }
    }
}


impl ActivityProcessor for ShiftProcessor {
    fn process(&self, activity: &activity::Activity) -> activity::Activity {
        let start = activity.start + self.shift;
        let end = activity.end.map(|end| end + self.shift);

        activity::Activity {
            start,
            end,
            project: activity.project.clone(),
            description: activity.description.clone(),
        }
    }
}

pub fn process_activities(
    activities: Vec<&activity::Activity>,
    processors: ProcessorList,
) -> Vec<activity::Activity> {
    activities
        .into_iter()
        .cloned()
        .map(|activity| {
            processors
                .iter()
                .fold(activity, |activity, processor| processor.process(&activity))
        })
        .collect()
}
