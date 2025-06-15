// test4.ts - A test for teams and pipelines.

@team({
    name: "PlanningTeam",
    description: "A team of agents for planning.",
    members: ["PlanningAgent", "ReminderAgent"]
})
export class PlanningTeam {
    // Team implementation
}

@pipeline({
    name: "PlanningPipeline",
    description: "A pipeline that uses the planning team."
})
export class PlanningPipeline {
    // Pipeline implementation
} 