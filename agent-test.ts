function agent(options: any) {
    return function (target: any) {
        // no-op
    };
}

@agent({
    name: "myAgent",
    description: "A test agent that uses myTool.",
    tools: ["myTool"]
})
export class MyAgent {
    // Agent implementation...
} 