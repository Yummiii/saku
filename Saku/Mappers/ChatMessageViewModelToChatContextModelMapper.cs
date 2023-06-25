using AutoMapper;
using Lina.DynamicMapperConfiguration.Abstracts;
using Saku.Models;
using Saku.ViewModels;

namespace Saku.Mappers;

public class ChatMessageViewModelToChatContextModelMapper : DynamicMapperProfile<ChatMessageViewModel, ChatContextModel>
{
    protected override void Map(IMappingExpression<ChatMessageViewModel, ChatContextModel> mappingExpression)
    {
        mappingExpression.ForCtorParam(nameof(ChatContextModel.IsPresentInCurrentContext), x => x.MapFrom(_ => true));
        mappingExpression.ForCtorParam(nameof(ChatContextModel.ChannelId), x => x.MapFrom(_ => 0UL));
    }
}