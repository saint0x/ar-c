/**
 * @file declarations.d.ts
 * @description Provides ambient type definitions for the Aria SDK decorators.
 * This allows the TypeScript compiler (`tsc`) to understand the shape of
 * our decorators during the pre-flight check, even though their actual
 * implementation is a no-op at compile time.
 */

/**
 * Declares the shape of the options for the @tool decorator.
 */
interface ToolOptions {
    name: string;
    description: string;
}

/**
 * Declares the shape of the options for the @agent decorator.
 */
interface AgentOptions {
    name: string;
    description: string;
    tools: string[];
}

/**
 * Declares the ambient `tool` decorator factory for tsc.
 */
declare function tool(options: ToolOptions): (target: any, propertyKey: string, descriptor: PropertyDescriptor) => void;

/**
 * Declares the ambient `agent` decorator factory for tsc.
 */
declare function agent(options: AgentOptions): (target: any) => void; 