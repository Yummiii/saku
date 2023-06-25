using AutoMapper;
using Lina.DynamicMapperConfiguration.Abstracts;
using Saku.Models;
using Saku.ViewModels;

namespace Saku.Mappers;

public class
    InputChatMessageViewModelToChatMessageViewModelMapper : DynamicMapperProfile<InputChatMessageViewModel,
        ChatMessageViewModel>
{
    protected override void Map(IMappingExpression<InputChatMessageViewModel, ChatMessageViewModel> mappingExpression)
    {
        mappingExpression.ForCtorParam(nameof(ChatMessageViewModel.ChatType), x => x.MapFrom(_ => ChatType.User));
    }
}