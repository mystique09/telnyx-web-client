export function conversationIdFromPath(url: string): string | null {
  const path = (url.split("?")[0] ?? "").replace(/\/+$/, "");
  if (!path.startsWith("/conversations/")) {
    return null;
  }

  const idSegment = path.slice("/conversations/".length).split("/")[0];
  const id = decodeURIComponent(idSegment ?? "");

  return id || null;
}

export function replaceConversationPath(conversationId: string | null) {
  if (typeof window === "undefined") {
    return;
  }

  const nextPath = conversationId
    ? `/conversations/${encodeURIComponent(conversationId)}`
    : "/conversations";
  window.history.pushState({}, "", nextPath);
}
