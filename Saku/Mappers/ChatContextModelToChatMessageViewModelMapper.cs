using AutoMapper;
using Lina.DynamicMapperConfiguration.Abstracts;
using Saku.Models;
using Saku.ViewModels;

namespace Saku.Mappers;

public class
    ChatContextModelToChatMessageViewModelMapper : DynamicMapperProfile<ChatContextModel, ChatMessageViewModel>
{
    protected override void Map(IMappingExpression<ChatContextModel, ChatMessageViewModel> mappingExpression)
    {
    }
}