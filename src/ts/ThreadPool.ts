
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
        runningFlag: boolean;
        runningProcessorCount: number;
        tasks: any[];
        size: any;
        constructor({
            size,
            tasks
        }) {
            this.size = size;

            this.tasks = [];
            this.addTask(...tasks);

            this.runningFlag = false;
            this.runningProcessorCount = 0;
        }

        isRunning() {
            return this.runningFlag || this.runningProcessorCount > 0;
        }

        start() {
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

        addTask(...tasks) {
            tasks.forEach(task => {
                if (task instanceof Task) {
                    this.tasks.push(task);
                }
                else {
                    console.error('expected to be instanceof Task');
                }
            })
        }

        processTask() {
            if (!this.runningFlag) {
                this.runningProcessorCount--;
                console.log('stop');
                return;
            }

            const task: any = this.tasks.shift();  //这里移除掉了 得修改一下 先修改状态 然后根据检索状态 决定去留 

            if (task) {
                const prom = task.processor(task.params);
                if (prom instanceof Promise) {
                    let cb;
                    prom.then(data => {
                        cb = task.callback(data);
                    }).finally(() => {
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