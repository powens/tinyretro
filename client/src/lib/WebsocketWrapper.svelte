<script lang="ts">
    // import { writable } from 'svelte/store';
    import { setContext, onMount, onDestroy } from 'svelte';
    import type { AllActions, Board } from '../lib/BoardState.svelte';
    import RetroBoard from './RetroBoard.svelte';

    const SOCKET_STATE = 'socketState';
    const SEND_ACTION = 'sendAction';

    let socketState = $state('disconnected');
    let sendAction = $state<(action: AllActions) => void>(() => {
        console.error('sendAction not initialized');
    });

    setContext(SOCKET_STATE, () => socketState);
    setContext(SEND_ACTION, () => sendAction);
    // const boardState = writable<Board|undefined>(undefined);

    let boardState = $state({}) as Board;
    // let { children } = $props();

    let socket = $state<WebSocket>();
    
    onMount(() => {
        socket = new WebSocket('ws://localhost:3000/ws');

        socket.addEventListener('open', () => {
            console.debug('Connected to server');
            socketState = 'connected';
        });

        socket.addEventListener('close', () => {
            console.debug('Disconnected from server');
            socketState = 'disconnected';
        });

        socket.addEventListener('error', (event) => {
            console.error('Error', event);
        });

        socket.addEventListener('message', (event) => {
            console.debug('Message from server', event.data);
            try {
                boardState = JSON.parse(event.data);
            } catch (error) {
                console.error('Error parsing JSON', error);
            }
        });

        sendAction = (action: AllActions) => {
            if (!socket) {
                console.error('Socket not initialized');
                return;
            }
            if (socket.readyState !== WebSocket.OPEN) {
                console.error('Socket not open');
                return;
            }
            console.debug('Sending message', action);
            socket.send(JSON.stringify(action));
        };
    });

    onDestroy(() => {
        if (!socket) { return ;}
            socket.close();
            socketState = 'disconnected';
    });

</script>

<div>
    <!-- {@render children?.()} -->
    {#if !boardState.title}
        <p>Loading...</p>
    {:else}

        <RetroBoard boardState={boardState} />
    {/if}
</div>