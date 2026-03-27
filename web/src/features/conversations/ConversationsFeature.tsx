import { PenSquare } from "lucide-react";

import { WorkspaceShell } from "@/components/workspace-shell";
import { Button } from "@/components/ui/button";
import { ConversationMainPanel } from "./components/ConversationMainPanel";
import { ConversationsSidebar } from "./components/ConversationsSidebar";
import { CreateConversationDialog } from "./components/CreateConversationDialog";
import { useConversationsController } from "./hooks/use-conversations-controller";

function ConversationsFeature() {
  const controller = useConversationsController();

  return (
    <WorkspaceShell
      activeView="conversations"
      title="Active conversations"
      description="Move between live threads, load older messages when needed, and keep the response surface anchored on the current conversation."
      isLoggingOut={controller.isLoggingOut}
      onLogout={controller.logout}
      scrollMode="content"
      actions={
        <CreateConversationDialog
          open={controller.isCreateConversationDialogOpen}
          onOpenChange={controller.openCreateConversationDialog}
          phoneNumbers={controller.phoneNumbers}
          fromPhoneNumberId={controller.fromPhoneNumberId}
          onFromPhoneNumberIdChange={controller.setFromPhoneNumberId}
          recipientPhoneInput={controller.recipientPhoneInput}
          onRecipientPhoneInputChange={controller.setRecipientPhoneInput}
          onCreateConversation={controller.createConversation}
          isCreatingConversation={controller.isCreatingConversation}
          trigger={
            <Button className="h-11 rounded-full px-5 shadow-sm">
              <PenSquare className="size-4" />
              New conversation
            </Button>
          }
        />
      }
      sidebarContent={
        <ConversationsSidebar
          conversations={controller.sortedConversations}
          selectedConversationId={controller.selectedConversationId}
          onSelectConversation={controller.selectConversation}
          deletingConversationId={controller.deletingConversationId}
          onDeleteConversation={controller.deleteConversation}
        />
      }
    >
      <div className="flex flex-1 flex-col overflow-hidden">
        <ConversationMainPanel
          hasConversations={controller.hasConversations}
          selectedConversation={controller.selectedConversation}
          selectedPhoneNumber={controller.selectedPhoneNumber}
          visibleMessages={controller.visibleMessages}
          nextCursor={controller.nextCursor}
          onLoadOlderMessages={controller.loadOlderMessages}
          messageDraft={controller.messageDraft}
          onMessageDraftChange={controller.setMessageDraft}
          onSendMessage={controller.sendMessage}
          onComposerKeyDown={controller.composerKeyDown}
          isSendingMessage={controller.isSendingMessage}
          sentMedia={controller.sentMedia}
        />
      </div>
    </WorkspaceShell>
  );
}

export default ConversationsFeature;
