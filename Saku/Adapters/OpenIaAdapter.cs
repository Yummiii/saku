using Lina.DynamicServicesProvider.Attributes;
using OpenAI_API;
using OpenAI_API.Chat;
using Saku.Adapters.Interfaces;
using Saku.ViewModels;
using Saku.ViewModels.Interfaces;

namespace Saku.Adapters;

[Adapter(typeof(IOpenIaAdapter))]
public class OpenIaAdapter : IOpenIaAdapter
{
    private readonly OpenAIAPI _openAiApi;

    public OpenIaAdapter(ISakuConfig sakuConfig)
    {
        _openAiApi = new OpenAIAPI(sakuConfig.OpenIaToken);
    }

    public async Task<IEnumerable<ChatMessageViewModel>> SendChat(IEnumerable<ChatMessageViewModel> chats,
        GptModelType model = GptModelType.Gpt35Turbo)
    {
        var chatsMessages = chats.ToList();
        
        var conversation = _openAiApi.Chat.CreateConversation(new ChatRequest
        {
            Model = model switch
            {
                GptModelType.Gpt40 => "gpt-4",
                GptModelType.Gpt35Turbo => "gpt-3.5-turbo"
            }
        });

        foreach (var chat in chatsMessages)
        {
            switch (chat.ChatType)
            {
                case ChatType.User:
                    if(chat.UserName is not null)
                        conversation.AppendUserInputWithName(chat.UserName, chat.Message);
                    else
                        conversation.AppendUserInput(chat.Message);
                    break;
                case ChatType.System:
                    conversation.AppendSystemMessage(chat.Message);
                    break;
                case ChatType.Chat:
                    conversation.AppendExampleChatbotOutput(chat.Message);
                    break;
                default:
                    throw new ArgumentOutOfRangeException(nameof(chat.ChatType), "ChatType is invalid");
            }
        }
        
        var response = await conversation.GetResponseFromChatbotAsync();
        chatsMessages.Add(new ChatMessageViewModel(ChatType.Chat, response));

        return chatsMessages;
    }
}