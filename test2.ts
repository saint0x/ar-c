/**
 * @module test2.ts
 * @description A more complex test case for the Aria compiler.
 *
 * This file includes multiple tools and agents to test the compiler's ability
 * to extract multiple entities from a single file and eventually handle
 * dependencies between them.
 */

// --- Decorator Factories (mock implementation) ---

function tool(options: any) {
    return function (target: any, propertyKey: string, descriptor: PropertyDescriptor) {
        // no-op
    };
}

function agent(options: any) {
    return function (target: any) {
        // no-op
    };
}


// --- Tool Definitions ---

@tool({
    name: "getWeather",
    description: "Fetches the current weather for a given location."
})
export function getWeather(params: { location: string }): { temperature: number; condition: string } {
    console.log(`Fetching weather for ${params.location}...`);
    return { temperature: 72, condition: "Sunny" };
}

@tool({
    name: "scheduleMeeting",
    description: "Schedules a meeting on the user's calendar."
})
export function scheduleMeeting(params: { topic: string; time: Date; attendees: string[] }): { success: boolean; meetingId: string } {
    console.log(`Scheduling meeting about '${params.topic}'...`);
    return { success: true, meetingId: `cal-${Date.now()}` };
}


// --- Agent Definitions ---

@agent({
    name: "PlanningAgent",
    description: "An agent that can plan events by checking the weather and scheduling meetings.",
    tools: ["getWeather", "scheduleMeeting"]
})
export class PlanningAgent {
    constructor() {
        // Initialization logic for the planning agent
    }

    async planTrip(destination: string, meetingTopic: string) {
        // In a real scenario, this agent would use an LLM to reason and call tools.
        // const weather = getWeather({ location: destination });
        // const meeting = scheduleMeeting({ topic: meetingTopic, time: new Date(), attendees: ["user"] });
        console.log("Planning trip...");
    }
}

@agent({
    name: "ReminderAgent",
    description: "A simple agent that sets reminders by scheduling them on the calendar.",
    tools: ["scheduleMeeting"]
})
export class ReminderAgent {
    setReminder(thingToRemember: string) {
        // const reminder = scheduleMeeting({ topic: `Reminder: ${thingToRemember}`, time: new Date(), attendees: ["user"] });
        console.log("Setting reminder...");
    }
} 