export interface LibreInfo {
    pid: number;
    status: 'Running' | 'Stopped';
    stopped_status: number | null;
};
