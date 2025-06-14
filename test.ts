function tool(options: any) {
    return function (target: any, propertyKey: string, descriptor: PropertyDescriptor) {
        // no-op
    };
}

@tool({
    name: "myTool",
    description: "A test tool"
})
export function myTool(params: { message: string }): void {
    console.log(`Executing myTool with message: ${params.message}`);
} 