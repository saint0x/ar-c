export class MyToolContainer {
@tool({
    name: "myTool",
    description: "A test tool"
})
    myTool(params: { message: string }): void {
    console.log(`Executing myTool with message: ${params.message}`);
    }
}

@agent({
    name: "myAgent",
    description: "A test agent that uses myTool.",
    tools: ["myTool"]
})
export class MyAgent {
    // This agent is now aware of myTool.
    // The runtime would be responsible for wiring them up.
} 