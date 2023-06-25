using AutoMapper;
using Lina.DynamicServicesProvider.Attributes;
using Saku.Models;
using Saku.Repositories.Interfaces;
using Saku.Services.Interfaces;
using Saku.ViewModels;

namespace Saku.Services;

[Service(typeof(IChannelService))]
public class ChannelService : IChannelService
{
    private readonly IChannelRepository _channelRepository;
    private readonly IMapper _mapper;
    private readonly IUnitOfWork _unitOfWork;

    public ChannelService(IChannelRepository channelRepository, IMapper mapper, IUnitOfWork unitOfWork)
    {
        _channelRepository = channelRepository;
        _mapper = mapper;
        _unitOfWork = unitOfWork;
    }

    public async ValueTask ChangeChannelState(ChannelRegisterViewModel register)
    {
        var channelModel = _mapper.Map<ChannelModel>(register);
        if (!await _channelRepository.HasChannel(register.DiscordChannelId))
        {
            await _channelRepository.Add(channelModel);
        }
        else
        {
            _channelRepository.Update(channelModel);
        }

        await _unitOfWork.SaveChanges();
    }

    public async Task<ChannelViewModel?> GetByDiscordId(ulong discordChannelId)
    {
        var channel = await _channelRepository.GetByDiscordId(discordChannelId);

        return channel is null ? null : _mapper.Map<ChannelViewModel>(channel);
    }
}