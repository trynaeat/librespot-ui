import { useQuery } from '@tanstack/react-query';
import { LibreInfo } from '../types/librespot';

function useLibreInfo () {
    return useQuery({
        queryKey: ['info'],
        queryFn: async(): Promise<LibreInfo> => {
            const response = await fetch(`/api/librespot/status`);
            return await response.json();
        },
        enabled: true,
        refetchInterval: 30 * 1000,
    })

}

export {
    useLibreInfo,
};