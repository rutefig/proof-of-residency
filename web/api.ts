export async function createProverSession(): Promise<{ session_id: string, prover_port: number }> {
    const response = await fetch('http://localhost:8080/session', {
        method: 'POST',
    });
    if (!response.ok) {
        throw new Error('Failed to create prover session');
    }
    return response.json();
}

export async function cleanupProverSession(sessionId: string): Promise<void> {
    await fetch(`http://localhost:8080/session/${sessionId}`, {
        method: 'DELETE',
    });
}