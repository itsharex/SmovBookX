
export module ThreadPool {
    export class Task {
        callback: (any) => void;
        params: any;
        processor: (any) => Promise<unknown>;
        constructor({ params, processor, callback }) {
            this.params = params;
            if (typeof processor === 'function') {
                this.processor = processor;
            }
            else {
                throw new Error('processor must be a funtion');
            }

            if (typeof callback === 'function') {
                this.callback = callback || (() => { });
            }
            else {
                throw new Error('callback must be a funtion');
            }
        }
    }

    // 异步池
    export class FixedThreadPool {
        runningFlag: any;
        runningProcessorCount: number;
        tasks: any[];
        size: any;
        index: number;
        constructor({ size, tasks,runningFlag }) {
            this.size = size;

            this.tasks = [];
            this.addTasks(...tasks);

            this.runningFlag = runningFlag;
            this.runningProcessorCount = 0;

            this.index = 0;
        }

        isRunning() {
            return this.runningFlag || this.runningProcessorCount > 0;
        }

        start() {
            // console.log(this.runningFlag);
            if (this.isRunning()) {
                return;
            }

            this.runningFlag = true;

            let i = this.size;
            while (i--) {
                this.processTask();
                this.runningProcessorCount++;
            }
        }

        stop() {
            this.runningFlag = false;
        }

        addTasks(...tasks) {
            tasks.forEach(task => {
                if (task instanceof Task) {
                    this.tasks.push(task);
                    // this.status[task.params.id] = 0;
                }
                else {
                    console.error('expected to be instanceof Task');
                }
            })
        }
        addTask(task) {
            if (task instanceof Task) {
                this.tasks.push(task);
                // this.status[task.params.id] = 0;
            }
            else {
                console.error('expected to be instanceof Task');
            }
        }

        processTask() {
            if (!this.runningFlag) {
                this.runningProcessorCount--;
                return;
            }
            //console.log(this.tasks.length)
            //const task: any = this.tasks.shift();  //这里移除掉了 得修改一下 先修改状态 然后根据检索状态 决定去留 
            const task: any = this.tasks[this.index];   //这里每次都是获取第一位 第一个执行的时候 还没删除呢 需要修改获取方式
            if (task) {
                this.index++;
                task.params.status = 1;
                const prom = task.processor(task.params);
                if (prom instanceof Promise) {
                    let cb;
                    prom.then(data => {
                        cb = task.callback(data);
                    }).finally(() => {
                        //this.tasks.shift();
                        //this.tasks.splice(nowIndex,1);  //在这出现了下标的变动
                        task.params.status = 2;
                        if (cb instanceof Promise) {
                            cb.finally(() => {
                                this.processTask();
                            });
                        }
                        else {
                            this.processTask();
                        }

                    });
                }
            }
            else {
                setTimeout(() => {
                    this.processTask();
                }, 500);
            }
        }
    }
}