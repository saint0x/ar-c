export class MyToolbox {
    @tool({
        name: "getWeather",
        description: "Fetches the current weather for a given location."
    })
    getWeather(params) {
        console.log(`Fetching weather for ${params.location}...`);
        return {
            temperature: 72,
            condition: "Sunny"
        };
    }
    @tool({
        name: "scheduleMeeting",
        description: "Schedules a meeting on the user's calendar."
    })
    scheduleMeeting(params) {
        console.log(`Scheduling meeting about '${params.topic}'...`);
        return {
            success: true,
            meetingId: `cal-${Date.now()}`
        };
    }
}
@agent({
    name: "PlanningAgent",
    description: "An agent that can plan events by checking the weather and scheduling meetings.",
    tools: [
        "getWeather",
        "scheduleMeeting"
    ]
})
export class PlanningAgent {
    constructor(){}
    async planTrip(destination, meetingTopic) {
        console.log("Planning trip...");
    }
}
@agent({
    name: "ReminderAgent",
    description: "A simple agent that sets reminders by scheduling them on the calendar.",
    tools: [
        "scheduleMeeting"
    ]
})
export class ReminderAgent {
    setReminder(thingToRemember) {
        console.log("Setting reminder...");
    }
}
