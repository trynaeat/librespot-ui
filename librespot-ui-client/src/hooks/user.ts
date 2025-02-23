import { useQuery } from '@tanstack/react-query';
import { User } from '../types/user';

export class UserError extends Error {
    private _status: number;
    private _msg: string;

    public get status() {
        return this._status;
    }

    public get msg() {
        return this._msg;
    }

    constructor (status: number, msg: string) {
        super(msg);
        this._status = status;
        this._msg = msg;
    }
}

export function useUser () {
    return useQuery({
        queryKey: ['user'],
        queryFn: async(): Promise<User | null> => {
            const response = await fetch('/api/auth/spotify/userinfo');
            // Not logged in, just return null
            if (response.status === 307) {
                return null;
            } else if (response.status !== 200) {
                throw new UserError(response.status, "Error fetching user");
            }
            return response.json();
        },
        gcTime: 0,
    });
}