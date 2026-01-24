import React, { useState, useEffect } from 'react';
import { DragDropContext, Droppable, Draggable } from 'react-beautiful-dnd';

interface Task {
  id: string;
  title: string;
  description: string;
  priority: 'low' | 'medium' | 'high' | 'critical';
  status: 'pending' | 'in_progress' | 'completed';
  assignedTo: string | null;
  createdAt: Date;
  dueDate?: Date;
  tags: string[];
}

interface Agent {
  id: string;
  name: string;
  capacity: number;
  assignedTasks: string[];
}

interface TaskOrchestratorProps {}

const TaskOrchestrator: React.FC<TaskOrchestratorProps> = () => {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [agents, setAgents] = useState<Agent[]>([]);
  const [loading, setLoading] = useState(true);

  // Generate mock data for demonstration
  useEffect(() => {
    const mockTasks: Task[] = [
      {
        id: 'task-1',
        title: 'Analyze user sentiment patterns',
        description: 'Review recent user interactions to identify emotional patterns and context triggers',
        priority: 'high',
        status: 'pending',
        assignedTo: null,
        createdAt: new Date(Date.now() - 1000 * 60 * 60 * 3),
        dueDate: new Date(Date.now() + 1000 * 60 * 60 * 12),
        tags: ['analysis', 'sentiment']
      },
      {
        id: 'task-2',
        title: 'Optimize language processing module',
        description: 'Refactor core NLP module to improve response time and reduce token usage',
        priority: 'medium',
        status: 'in_progress',
        assignedTo: 'agent-2',
        createdAt: new Date(Date.now() - 1000 * 60 * 60 * 24),
        dueDate: new Date(Date.now() + 1000 * 60 * 60 * 5),
        tags: ['development', 'optimization']
      },
      {
        id: 'task-3',
        title: 'Generate creative response templates',
        description: 'Create new response templates for common relationship scenarios',
        priority: 'low',
        status: 'pending',
        assignedTo: null,
        createdAt: new Date(Date.now() - 1000 * 60 * 60 * 10),
        tags: ['creative', 'content']
      },
      {
        id: 'task-4',
        title: 'Security audit on memory storage',
        description: 'Review security protocols for L5 procedural memory storage',
        priority: 'critical',
        status: 'pending',
        assignedTo: null,
        createdAt: new Date(Date.now() - 1000 * 60 * 30),
        dueDate: new Date(Date.now() + 1000 * 60 * 60 * 2),
        tags: ['security', 'memory']
      },
      {
        id: 'task-5',
        title: 'Catalog personal user preferences',
        description: 'Update user preference vectors based on recent interactions',
        priority: 'medium',
        status: 'completed',
        assignedTo: 'agent-1',
        createdAt: new Date(Date.now() - 1000 * 60 * 60 * 48),
        tags: ['analytics', 'personalization']
      }
    ];

    const mockAgents: Agent[] = [
      {
        id: 'agent-1',
        name: 'KinkResearcher',
        capacity: 3,
        assignedTasks: ['task-5']
      },
      {
        id: 'agent-2',
        name: 'CodeOptimizer',
        capacity: 2,
        assignedTasks: ['task-2']
      },
      {
        id: 'agent-3',
        name: 'CreativeWriter',
        capacity: 4,
        assignedTasks: []
      }
    ];

    setTasks(mockTasks);
    setAgents(mockAgents);
    setLoading(false);
  }, []);

  const handleDragEnd = (result: any) => {
    const { destination, source, draggableId } = result;
    
    // Dropped outside a droppable area
    if (!destination) return;
    
    // Dropped in same position
    if (
      destination.droppableId === source.droppableId &&
      destination.index === source.index
    ) {
      return;
    }
    
    // Handle task assignment to agent
    if (destination.droppableId.startsWith('agent-')) {
      const agentId = destination.droppableId;
      const taskId = draggableId;
      
      // Update agents with new task assignment
      setAgents(prevAgents => {
        const updatedAgents = prevAgents.map(agent => {
          // If this is the destination agent, add task
          if (agent.id === agentId) {
            return {
              ...agent,
              assignedTasks: [...agent.assignedTasks, taskId]
            };
          }
          
          // If task was previously assigned to another agent, remove it
          if (agent.assignedTasks.includes(taskId)) {
            return {
              ...agent,
              assignedTasks: agent.assignedTasks.filter(id => id !== taskId)
            };
          }
          
          return agent;
        });
        
        return updatedAgents;
      });
      
      // Update task with new assignment and status
      setTasks(prevTasks => {
        return prevTasks.map(task => {
          if (task.id === taskId) {
            return {
              ...task,
              assignedTo: agentId,
              status: 'in_progress'
            };
          }
          return task;
        });
      });
    }
    
    // Handle re-ordering within pending tasks
    if (source.droppableId === 'pending-tasks' && destination.droppableId === 'pending-tasks') {
      const newTasks = Array.from(tasks);
      const pendingTasks = newTasks.filter(t => t.status === 'pending' && !t.assignedTo);
      const [movedTask] = pendingTasks.splice(source.index, 1);
      pendingTasks.splice(destination.index, 0, movedTask);
      
      // Rebuild the full task list with the reordered pending tasks
      const otherTasks = newTasks.filter(t => t.status !== 'pending' || t.assignedTo);
      setTasks([...pendingTasks, ...otherTasks]);
    }
  };

  const getPriorityStyle = (priority: Task['priority']) => {
    switch (priority) {
      case 'low': return 'bg-blue-500/20 text-blue-300';
      case 'medium': return 'bg-yellow-500/20 text-yellow-300';
      case 'high': return 'bg-orange-500/20 text-orange-300';
      case 'critical': return 'bg-red-500/20 text-red-300';
      default: return 'bg-gray-500/20 text-gray-300';
    }
  };

  const getPriorityIcon = (priority: Task['priority']) => {
    switch (priority) {
      case 'low': return 'low_priority';
      case 'medium': return 'priority';
      case 'high': return 'priority_high';
      case 'critical': return 'error';
      default: return 'help';
    }
  };

  const formatTimeAgo = (date: Date): string => {
    const now = new Date();
    const diff = now.getTime() - date.getTime();
    
    const minutes = Math.floor(diff / (1000 * 60));
    const hours = Math.floor(diff / (1000 * 60 * 60));
    const days = Math.floor(diff / (1000 * 60 * 60 * 24));
    
    if (days > 0) return `${days}d ago`;
    if (hours > 0) return `${hours}h ago`;
    if (minutes > 0) return `${minutes}m ago`;
    return 'just now';
  };

  if (loading) {
    return (
      <div className="bg-panel-dark border border-border-dark rounded-lg p-4">
        <div className="animate-pulse flex flex-col gap-4">
          <div className="h-6 bg-gray-700 rounded w-1/4"></div>
          <div className="h-32 bg-gray-700 rounded"></div>
          <div className="h-32 bg-gray-700 rounded"></div>
        </div>
      </div>
    );
  }

  return (
    <div className="bg-panel-dark border border-border-dark rounded-lg p-4">
      {/* Header */}
      <div className="flex justify-between items-center mb-4">
        <div className="flex items-center gap-2">
          <span className="material-symbols-outlined text-lg text-primary">task</span>
          <h2 className="font-bold text-white">Task Orchestrator</h2>
        </div>
        <button className="text-xs flex items-center gap-1 px-2 py-1 rounded bg-primary/20 text-primary hover:bg-primary/30 transition-colors">
          <span className="material-symbols-outlined text-[16px]">add</span>
          New Task
        </button>
      </div>

      <DragDropContext onDragEnd={handleDragEnd}>
        <div className="flex flex-col lg:flex-row gap-4">
          {/* Pending Tasks */}
          <div className="flex-1">
            <h3 className="text-sm font-semibold text-gray-400 mb-2 flex items-center gap-1">
              <span className="material-symbols-outlined text-[16px]">pending_actions</span>
              Pending Tasks
            </h3>
            
            <Droppable droppableId="pending-tasks">
              {(provided) => (
                <div
                  ref={provided.innerRef}
                  {...provided.droppableProps}
                  className="space-y-2 min-h-[200px]"
                >
                  {tasks.filter(task => task.status === 'pending' && !task.assignedTo).map((task, index) => (
                    <Draggable key={task.id} draggableId={task.id} index={index}>
                      {(provided) => (
                        <div
                          ref={provided.innerRef}
                          {...provided.draggableProps}
                          {...provided.dragHandleProps}
                          className="border border-border-dark bg-background-dark hover:bg-panel-dark p-3 rounded-lg transition-colors cursor-move"
                        >
                          <div className="flex justify-between items-start mb-2">
                            <span className="font-semibold text-white">{task.title}</span>
                            <div className={`text-xs px-2 py-0.5 rounded-full flex items-center gap-1 ${getPriorityStyle(task.priority)}`}>
                              <span className="material-symbols-outlined text-[14px]">{getPriorityIcon(task.priority)}</span>
                              <span>{task.priority}</span>
                            </div>
                          </div>
                          <p className="text-sm text-gray-400 mb-2 line-clamp-2">{task.description}</p>
                          <div className="flex justify-between items-center">
                            <div className="flex gap-1 flex-wrap">
                              {task.tags.map((tag, i) => (
                                <span key={i} className="text-[10px] px-1.5 py-0.5 bg-gray-700 rounded text-gray-400">
                                  {tag}
                                </span>
                              ))}
                            </div>
                            <span className="text-xs text-gray-500">{formatTimeAgo(task.createdAt)}</span>
                          </div>
                        </div>
                      )}
                    </Draggable>
                  ))}
                  {provided.placeholder}
                  
                  {tasks.filter(task => task.status === 'pending' && !task.assignedTo).length === 0 && (
                    <div className="border border-dashed border-gray-700 rounded-lg p-4 flex items-center justify-center">
                      <span className="text-sm text-gray-500">No pending tasks</span>
                    </div>
                  )}
                </div>
              )}
            </Droppable>
          </div>

          {/* Agent Assignment Columns */}
          <div className="flex-1">
            <h3 className="text-sm font-semibold text-gray-400 mb-2 flex items-center gap-1">
              <span className="material-symbols-outlined text-[16px]">smart_toy</span>
              Agent Assignments
            </h3>
            
            <div className="space-y-4">
              {agents.map(agent => (
                <div key={agent.id} className="border border-border-dark rounded-lg p-3 bg-background-dark">
                  <div className="flex justify-between items-center mb-2">
                    <div className="flex items-center gap-2">
                      <div className="size-2 rounded-full bg-green-500"></div>
                      <span className="font-semibold text-white">{agent.name}</span>
                    </div>
                    <span className="text-xs text-gray-500">
                      {agent.assignedTasks.length}/{agent.capacity} tasks
                    </span>
                  </div>

                  <Droppable droppableId={agent.id}>
                    {(provided) => (
                      <div
                        ref={provided.innerRef}
                        {...provided.droppableProps}
                        className={`mt-2 rounded-lg border border-dashed min-h-[100px] p-2 ${
                          agent.assignedTasks.length >= agent.capacity 
                            ? 'border-red-500/30 bg-red-900/10' 
                            : 'border-primary/30 bg-primary/5'
                        }`}
                      >
                        {tasks
                          .filter(task => agent.assignedTasks.includes(task.id))
                          .map((task, index) => (
                            <Draggable key={task.id} draggableId={task.id} index={index}>
                              {(provided) => (
                                <div
                                  ref={provided.innerRef}
                                  {...provided.draggableProps}
                                  {...provided.dragHandleProps}
                                  className="border border-border-dark bg-background-dark p-2 rounded mb-2 cursor-move"
                                >
                                  <div className="flex justify-between items-start">
                                    <span className="text-sm text-white">{task.title}</span>
                                    <div className={`text-[10px] px-1.5 rounded-full flex items-center gap-0.5 ${getPriorityStyle(task.priority)}`}>
                                      <span className="material-symbols-outlined text-[12px]">{getPriorityIcon(task.priority)}</span>
                                    </div>
                                  </div>
                                </div>
                              )}
                            </Draggable>
                        ))}
                        {provided.placeholder}
                        
                        {agent.assignedTasks.length == 0 && (
                          <div className="flex items-center justify-center h-full">
                            <span className="text-xs text-gray-500">Drag tasks here</span>
                          </div>
                        )}
                      </div>
                    )}
                  </Droppable>
                </div>
              ))}
            </div>
          </div>
        </div>
      </DragDropContext>

      {/* Completed Tasks (Collapsed Section) */}
      <div className="mt-6">
        <div className="flex items-center gap-2 mb-2 cursor-pointer select-none">
          <span className="material-symbols-outlined text-[16px] text-gray-400">expand_more</span>
          <h3 className="text-sm font-semibold text-gray-400 flex items-center gap-1">
            <span className="material-symbols-outlined text-[16px]">task_alt</span>
            Completed Tasks ({tasks.filter(t => t.status === 'completed').length})
          </h3>
        </div>
      </div>
    </div>
  );
};

export default TaskOrchestrator;