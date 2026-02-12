import { ConversationMainPanel } from "./components/ConversationMainPanel";
import { ConversationsSidebar } from "./components/ConversationsSidebar";
import { useConversationsController } from "./hooks/use-conversations-controller";

function ConversationsFeature() {
  const controller = useConversationsController();

  return (
    <div className="h-screen w-full bg-background">
      <div className="flex h-full w-full flex-col overflow-hidden bg-background md:flex-row">
        <ConversationsSidebar
          isLoggingOut={controller.isLoggingOut}
          onLogout={controller.logout}
          phoneNumbers={controller.phoneNumbers}
          conversations={controller.sortedConversations}
          selectedConversationId={controller.selectedConversationId}
          onSelectConversation={controller.selectConversation}
          isCreateConversationDialogOpen={controller.isCreateConversationDialogOpen}
          onOpenCreateConversationDialog={controller.openCreateConversationDialog}
          fromPhoneNumberId={controller.fromPhoneNumberId}
          onFromPhoneNumberIdChange={controller.setFromPhoneNumberId}
          conversationNameInput={controller.conversationNameInput}
          onConversationNameInputChange={controller.setConversationNameInput}
          recipientPhoneInput={controller.recipientPhoneInput}
          onRecipientPhoneInputChange={controller.setRecipientPhoneInput}
          onCreateConversation={controller.createConversation}
        />

        <main className="flex flex-1 flex-col overflow-hidden">
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
            sentMedia={controller.sentMedia}
          />
        </main>
      </div>
    </div>
  );
}

export default ConversationsFeature;
