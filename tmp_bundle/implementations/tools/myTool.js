export class MyToolContainer {
    @tool({
        name: "myTool",
        description: "A test tool"
    })
    myTool(params) {
        console.log(`Executing myTool with message: ${params.message}`);
    }
}
