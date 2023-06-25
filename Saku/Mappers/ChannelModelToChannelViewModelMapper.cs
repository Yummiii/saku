using AutoMapper;
using Lina.DynamicMapperConfiguration.Abstracts;
using Saku.Models;
using Saku.ViewModels;

namespace Saku.Mappers;

public class ChannelModelToChannelViewModelMapper : DynamicMapperProfile<ChannelModel, ChannelViewModel>
{
    protected override void Map(IMappingExpression<ChannelModel, ChannelViewModel> mappingExpression)
    {
    }
}