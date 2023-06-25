using AutoMapper;
using Lina.DynamicMapperConfiguration.Abstracts;
using Saku.Models;
using Saku.ViewModels;

namespace Saku.Mappers;

public class ChannelRegisterViewModelToChannelModelMapper : DynamicMapperProfile<ChannelRegisterViewModel, ChannelModel>
{
    protected override void Map(IMappingExpression<ChannelRegisterViewModel, ChannelModel> mappingExpression)
    {
        mappingExpression.ForCtorParam(nameof(ChannelModel.Model), x => x.MapFrom(_ => GptModelType.Gpt35Turbo));
        mappingExpression.ForCtorParam(nameof(ChannelModel.State),
            x => x.MapFrom(y => MapState(y.ChannelEnable, y.NoLogs)));
    }

    private static ChannelState MapState(bool enableChannel, bool noLog)
    {
        var state = enableChannel ? ChannelState.Enable : ChannelState.Disable;

        if (noLog)
            state |= ChannelState.NoLogs;

        return state;
    }
}