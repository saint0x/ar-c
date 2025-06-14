# Aria SDK: Complete Usage Examples

This document provides a comprehensive example of the Aria SDK, demonstrating the new decorator-based syntax for defining and orchestrating Tools, Agents, Teams, and Pipelines.

## TL;DR: Aria SDK Decorator-Based Syntax

**Core Philosophy**: Everything from `@aria/sdk` with clean decorators, full type safety, and zero config defaults.

**Key Patterns**:
- `@tool()` - Define reusable capabilities with structured I/O
- `@agent()` - Intelligent entities that use tools and LLMs
- `@team()` - Coordinated groups of agents with delegation strategies
- `@pipeline()` - Multi-step workflows with branching/loops/error handling
- `@memory()`, `@streaming()`, `@validation()` - Advanced features
- `@configure()` - Global/environment-specific configuration

**Benefits**:
- Single import, decorator-driven
- Auto JSON mode for tool-enabled agents
- Built-in streaming, memory, validation
- Full TypeScript inference and runtime validation
- Clean multi-line functions with proper error handling

---

## Complete Example: Research Assistant with Team & Pipeline

This example builds a complete research and publication workflow.

### 1. Global Configuration

First, we set up a global configuration for the application. The `@configure` decorator allows setting default LLM providers, database connections, streaming options, and more.

```typescript
import { 
  tool, agent, team, pipeline, configure,
  ToolResult, AgentResult, TeamResult, PipelineResult
} from '@aria/sdk';

// Global configuration
@configure({
  llm: { defaultModel: "gpt-4o-mini", temperature: 0.7 },
  database: { enabled: true },
  streaming: { enabled: true }
})
class ResearchApp {}
```

### 2. Tools

Tools are the fundamental building blocks. Here we define three tools: `webSearch` to find information, `writeReport` to save content, and `analyzeSentiment` for text analysis. Each tool has strongly-typed inputs and outputs.

```typescript
// --- Tool 1: Web Search ---
@tool({
  name: "webSearch",
  description: "Search the web for current information",
  inputs: {
    query: { type: "string", required: true },
    maxResults: { type: "number", default: 5, min: 1, max: 20 }
  },
  outputs: {
    results: "array",
    searchTime: "number"
  }
})
export async function webSearch(params: {
  query: string;
  maxResults?: number;
}): Promise<ToolResult<{
  results: Array<{ title: string; url: string; snippet: string }>;
  searchTime: number;
}>> {
  const startTime = Date.now();
  
  try {
    // Simulate web search
    const results = [
      {
        title: `Research on ${params.query}`,
        url: `https://example.com/search?q=${encodeURIComponent(params.query)}`,
        snippet: `Comprehensive information about ${params.query}...`
      }
    ];
    
    return {
      success: true,
      result: {
        results: results.slice(0, params.maxResults || 5),
        searchTime: Date.now() - startTime
      },
      metrics: {
        duration: Date.now() - startTime,
        startTime,
        endTime: Date.now()
      }
    };
  } catch (error) {
    return {
      success: false,
      error: error.message,
      metrics: {
        duration: Date.now() - startTime,
        startTime,
        endTime: Date.now()
      }
    };
  }
}

// --- Tool 2: Write Report ---
@tool({
  name: "writeReport",
  description: "Write a structured research report",
  inputs: {
    title: { type: "string", required: true },
    content: { type: "string", required: true },
    format: { type: "string", enum: ["markdown", "html", "pdf"], default: "markdown" }
  }
})
export async function writeReport(params: {
  title: string;
  content: string;
  format?: "markdown" | "html" | "pdf";
}): Promise<ToolResult<{
  filePath: string;
  wordCount: number;
}>> {
  try {
    const filePath = `./reports/${params.title.replace(/\s+/g, '-').toLowerCase()}.${params.format || 'md'}`;
    const wordCount = params.content.split(/\s+/).length;
    
    // Simulate file writing
    console.log(`Writing report: ${params.title} (${wordCount} words) to ${filePath}`);
    
    return {
      success: true,
      result: {
        filePath,
        wordCount
      }
    };
  } catch (error) {
    return {
      success: false,
      error: error.message
    };
  }
}

// --- Tool 3: Analyze Sentiment ---
@tool({
  name: "analyzeSentiment", 
  description: "Analyze sentiment and tone of text content",
  inputs: {
    text: { type: "string", required: true },
    includeEmotions: { type: "boolean", default: false }
  }
})
export async function analyzeSentiment(params: {
  text: string;
  includeEmotions?: boolean;
}): Promise<ToolResult<{
  sentiment: "positive" | "negative" | "neutral";
  confidence: number;
  emotions?: string[];
}>> {
  // Simulate sentiment analysis
  const sentiments = ["positive", "negative", "neutral"] as const;
  const sentiment = sentiments[Math.floor(Math.random() * sentiments.length)];
  
  return {
    success: true,
    result: {
      sentiment,
      confidence: 0.85,
      emotions: params.includeEmotions ? ["curiosity", "engagement"] : undefined
    }
  };
}
```

### 3. Agents

Agents are intelligent entities that use tools to accomplish tasks. We define a `ResearcherAgent` to gather information and a `WriterAgent` to compile it. The `@agent` decorator configures their behavior, and the `@task` decorator defines their specific actions.

```typescript
// --- Agent 1: Researcher ---
@agent({
  name: "ResearcherAgent",
  description: "Expert researcher specializing in comprehensive information gathering and analysis",
  task: "Conduct thorough research on any topic with proper citations and analysis",
  tools: ["webSearch", "analyzeSentiment"],
  
  llm: {
    model: "gpt-4o-mini",
    temperature: 0.3, // Lower for factual research
    maxTokens: 3000
  },
  
  systemPrompt: `You are an expert researcher. Always:
    - Search for multiple sources to verify information
    - Provide clear citations and source URLs  
    - Analyze the sentiment and reliability of sources
    - Structure findings logically
    - Flag any potential bias or limitations`,
    
  capabilities: ["research", "fact_checking", "source_analysis"],
  streaming: { enabled: true }
})
export class ResearcherAgent {
  @task({
    description: "Research a topic thoroughly with source analysis",
    inputs: {
      topic: { type: "string", required: true },
      depth: { type: "string", enum: ["overview", "detailed", "comprehensive"], default: "detailed" }
    },
    streaming: true
  })
  async researchTopic(
    params: {
      topic: string;
      depth?: "overview" | "detailed" | "comprehensive";
    },
    context: {
      onProgress?: (update: { status: string; progress: number }) => void;
    }
  ): Promise<AgentResult<{
    findings: string;
    sources: Array<{ url: string; reliability: string }>;
    sentiment: string;
    confidence: number;
  }>> {
    context.onProgress?.({ status: "Starting research", progress: 10 });
    
    // Use tools for research
    const searchResults = await this.useTool("webSearch", {
      query: params.topic,
      maxResults: params.depth === "comprehensive" ? 10 : 5
    });
    
    context.onProgress?.({ status: "Analyzing sources", progress: 50 });
    
    if (!searchResults.success) {
      return {
        success: false,
        error: "Failed to search for information"
      };
    }
    
    // Analyze sentiment of search results
    const sentimentResults = await this.useTool("analyzeSentiment", {
      text: searchResults.result.results.map(r => r.snippet).join(" "),
      includeEmotions: true
    });
    
    context.onProgress?.({ status: "Compiling findings", progress: 90 });
    
    return {
      success: true,
      result: {
        findings: `Research on "${params.topic}" found ${searchResults.result.results.length} sources. Key insights: ${searchResults.result.results.map(r => r.title).join(", ")}`,
        sources: searchResults.result.results.map(r => ({
          url: r.url,
          reliability: "verified" // Would be actual analysis
        })),
        sentiment: sentimentResults.success ? sentimentResults.result.sentiment : "neutral",
        confidence: sentimentResults.success ? sentimentResults.result.confidence : 0.5
      },
      metrics: {
        duration: 5000,
        startTime: Date.now() - 5000,
        endTime: Date.now(),
        toolCalls: 2
      }
    };
  }
}

// --- Agent 2: Writer ---
@agent({
  name: "WriterAgent", 
  description: "Professional writer specializing in clear, engaging content creation",
  task: "Transform research findings into well-structured, readable reports",
  tools: ["writeReport", "analyzeSentiment"],
  
  llm: {
    model: "gpt-4o-mini",
    temperature: 0.6, // Slightly higher for creative writing
    maxTokens: 4000
  },
  
  capabilities: ["writing", "editing", "formatting", "storytelling"]
})
export class WriterAgent {
  @task({
    description: "Create a professional report from research findings"
  })
  async createReport(params: {
    title: string;
    research: any;
    style: "academic" | "business" | "general";
  }): Promise<AgentResult<{
    report: string;
    filePath: string;
    wordCount: number;
  }>> {
    // Transform research into structured report
    const reportContent = `# ${params.title}\n\n## Executive Summary\n${params.research.findings}\n\n## Sources and Reliability  \n${params.research.sources.map(s => `- ${s.url} (${s.reliability})`).join('\n')}\n\n## Analysis\nSentiment: ${params.research.sentiment} (confidence: ${params.research.confidence})\n\n## Conclusions\nBased on the research findings...`;

    const writeResult = await this.useTool("writeReport", {
      title: params.title,
      content: reportContent,
      format: "markdown"
    });

    if (!writeResult.success) {
      return {
        success: false,
        error: "Failed to write report"
      };
    }

    return {
      success: true,
      result: {
        report: reportContent,
        filePath: writeResult.result.filePath,
        wordCount: writeResult.result.wordCount
      }
    };
  }
}
```

### 4. Team

Teams enable collaboration between agents. The `ResearchTeam` combines the `ResearcherAgent` and `WriterAgent`. The `@team` decorator defines the coordination strategy and delegation logic, while `@member` assigns agents to the team.

```typescript
@team({
  name: "ResearchTeam",
  description: "Collaborative research team combining investigation and writing expertise",
  
  strategy: {
    type: "sequential_collaboration",
    coordination: "llm_driven",
    maxParallelTasks: 2
  },
  
  delegation: {
    type: "capability_based",
    rules: [
      {
        condition: (task: string) => task.includes("research") || task.includes("investigate"),
        assignTo: ["ResearcherAgent"]
      },
      {
        condition: (task: string) => task.includes("write") || task.includes("report"),
        assignTo: ["WriterAgent"]
      }
    ]
  },
  
  memory: {
    shared: true,
    context: "research_projects"
  }
})
export class ResearchTeam {
  @member({ role: "lead", specialization: "research" })
  researcher = new ResearcherAgent();
  
  @member({ role: "specialist", specialization: "writing" })
  writer = new WriterAgent();
  
  @task({
    description: "Complete research project from investigation to final report",
    coordination: "sequential",
    streaming: true
  })
  async completeResearchProject(
    params: {
      topic: string;
      reportTitle: string;
      depth: "overview" | "detailed" | "comprehensive";
      style: "academic" | "business" | "general";
    },
    context: {
      onProgress?: (update: { member: string; status: string; progress: number }) => void;
    }
  ): Promise<TeamResult<{
    research: any;
    report: any;
    totalDuration: number;
  }>> {
    const startTime = Date.now();
    
    // Step 1: Research phase
    context.onProgress?.({ member: "researcher", status: "Starting research", progress: 10 });
    
    const researchResult = await this.researcher.researchTopic(
      { topic: params.topic, depth: params.depth },
      {
        onProgress: (update) => context.onProgress?.({
          member: "researcher",
          status: update.status,
          progress: 10 + (update.progress * 0.4) // 10-50% of total
        })
      }
    );
    
    if (!researchResult.success) {
      return {
        success: false,
        error: "Research phase failed",
        metrics: { totalDuration: Date.now() - startTime }
      };
    }
    
    // Step 2: Writing phase
    context.onProgress?.({ member: "writer", status: "Creating report", progress: 60 });
    
    const reportResult = await this.writer.createReport({
      title: params.reportTitle,
      research: researchResult.result,
      style: params.style
    });
    
    if (!reportResult.success) {
      return {
        success: false,
        error: "Report writing failed",
        metrics: { totalDuration: Date.now() - startTime }
      };
    }
    
    context.onProgress?.({ member: "team", status: "Project complete", progress: 100 });
    
    return {
      success: true,
      result: {
        research: researchResult.result,
        report: reportResult.result,
        totalDuration: Date.now() - startTime
      },
      metrics: {
        totalDuration: Date.now() - startTime,
        memberMetrics: {
          researcher: researchResult.metrics,
          writer: reportResult.metrics
        }
      }
    };
  }
}
```

### 5. Pipeline

Pipelines orchestrate complex, multi-step workflows. The `ResearchPublicationPipeline` defines a sequence of steps involving tools, teams, and conditional logic. The `@pipeline` decorator configures the overall workflow, while `@step` defines each stage, its dependencies, and data flow.

```typescript
@pipeline({
  name: "ResearchPublicationPipeline",
  description: "Complete pipeline from research request to published report",
  
  variables: {
    requestId: { type: "string", required: true },
    topic: { type: "string", required: true },
    priority: { type: "string", enum: ["low", "medium", "high"], default: "medium" },
    publishTarget: { type: "string", default: "internal" }
  },
  
  errorStrategy: {
    type: "retry_with_fallback",
    maxAttempts: 2,
    fallbackStrategy: "manual_intervention"
  },
  
  monitoring: {
    metrics: true,
    logging: "detailed"
  }
})
export class ResearchPublicationPipeline {
  @step({
    id: "validate_request",
    description: "Validate and sanitize research request",
    type: "tool",
    tool: "validateInput", // Would be defined elsewhere
    
    inputs: {
      topic: "$topic",
      priority: "$priority"
    },
    
    outputs: {
      validatedTopic: ".result.topic",
      estimatedDuration: ".result.duration"
    }
  })
  validateRequest!: PipelineStep;
  
  @step({
    id: "conduct_research",
    description: "Execute research using the research team",
    type: "team",
    team: "ResearchTeam",
    dependencies: ["validate_request"],
    
    inputs: {
      topic: "@validate_request.validatedTopic",
      reportTitle: `Research Report: @validate_request.validatedTopic`,
      depth: "detailed",
      style: "business"
    },
    
    outputs: {
      researchResults: ".result.research",
      reportData: ".result.report",
      teamMetrics: ".metrics"
    },
    
    timeout: 600000 // 10 minutes
  })
  conductResearch!: PipelineStep;
  
  @step({
    id: "quality_review",
    description: "Review report quality and fact-check",
    type: "agent",
    agent: "QualityReviewer", // Would be defined elsewhere
    dependencies: ["conduct_research"],
    
    condition: {
      expression: "$priority === 'high'",
      onFalse: "skip"
    },
    
    inputs: {
      report: "@conduct_research.reportData",
      sources: "@conduct_research.researchResults.sources"
    }
  })
  qualityReview!: PipelineStep;
  
  @step({
    id: "publish_report",
    description: "Publish the final report to the specified target",
    type: "tool", 
    tool: "publishContent", // Would be defined elsewhere
    dependencies: ["conduct_research", "quality_review"],
    
    inputs: {
      content: "@conduct_research.reportData.report",
      target: "$publishTarget",
      metadata: {
        requestId: "$requestId",
        topic: "$topic",
        qualityReviewed: "@quality_review ? true : false"
      }
    }
  })
  publishReport!: PipelineStep;
  
  @execute({
    description: "Execute the complete research publication pipeline",
    streaming: true
  })
  async executeResearchPipeline(
    params: {
      requestId: string;
      topic: string;
      priority?: "low" | "medium" | "high";
      publishTarget?: string;
    },
    context: {
      onProgress?: (update: { step: string; status: string; progress: number }) => void;
      onStepComplete?: (step: string, result: any) => void;
    }
  ): Promise<PipelineResult<{
    requestId: string;
    research: any;
    report: any;
    published: boolean;
    qualityReviewed: boolean;
  }>> {
    const startTime = Date.now();
    
    try {
      // Pipeline orchestration logic would be handled by the framework
      // This is a simplified representation of what the execution would look like
      
      context.onProgress?.({ step: "pipeline", status: "Starting pipeline", progress: 0 });
      
      // The actual step execution would be handled by the pipeline framework
      // based on the step definitions and dependencies
      
      return {
        success: true,
        result: {
          requestId: params.requestId,
          research: {}, // Would contain actual research results
          report: {},   // Would contain actual report data
          published: true,
          qualityReviewed: params.priority === "high"
        },
        steps: [
          { id: "validate_request", success: true, duration: 500 },
          { id: "conduct_research", success: true, duration: 15000 },
          { id: "quality_review", success: true, duration: 3000 },
          { id: "publish_report", success: true, duration: 1000 }
        ],
        metrics: {
          totalDuration: Date.now() - startTime,
          stepsExecuted: 4,
          stepsSkipped: 0,
          stepsParallel: 0
        }
      };
    } catch (error) {
      return {
        success: false,
        error: error.message,
        metrics: {
          totalDuration: Date.now() - startTime,
          stepsExecuted: 0,
          stepsSkipped: 0,
          stepsParallel: 0
        }
      };
    }
  }
}
```

### 6. Usage Example

Finally, this `main` function demonstrates how to execute the individual agents, the collaborative team, and the end-to-end pipeline.

```typescript
async function main() {
  // Execute individual agent
  const researcher = new ResearcherAgent();
  const researchResult = await researcher.researchTopic(
    { topic: "AI in healthcare", depth: "detailed" },
    { onProgress: (update) => console.log(`Research: ${update.status} - ${update.progress}%`) }
  );
  
  console.log("Research completed:", researchResult.success);
  
  // Execute team collaboration
  const team = new ResearchTeam();
  const teamResult = await team.completeResearchProject(
    {
      topic: "Quantum computing applications",
      reportTitle: "Quantum Computing in Industry",
      depth: "comprehensive",
      style: "business"
    },
    {
      onProgress: (update) => console.log(`${update.member}: ${update.status} - ${update.progress}%`)
    }
  );
  
  console.log("Team project completed:", teamResult.success);
  
  // Execute full pipeline
  const pipeline = new ResearchPublicationPipeline();
  const pipelineResult = await pipeline.executeResearchPipeline(
    {
      requestId: "req-001",
      topic: "Sustainable energy technologies",
      priority: "high",
      publishTarget: "public"
    },
    {
      onProgress: (update) => console.log(`Pipeline ${update.step}: ${update.status} - ${update.progress}%`),
      onStepComplete: (step, result) => console.log(`Step ${step} completed:`, result.success)
    }
  );
  
  console.log("Pipeline completed:", pipelineResult.success);
  console.log("Published report:", pipelineResult.result?.published);
}

// Run the example
main().catch(console.error); 