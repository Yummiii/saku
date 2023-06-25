using System.Text.RegularExpressions;
using AutoMapper;
using Lina.DynamicServicesProvider;
using Lina.DynamicServicesProvider.Attributes;
using Saku.Adapters.Interfaces;
using Saku.Models;
using Saku.Repositories.Interfaces;
using Saku.Services.Interfaces;
using Saku.ViewModels;

namespace Saku.Services;

[Service(typeof(IIaChatService))]
public class IaChatService : IIaChatService
{
    private readonly IOpenIaAdapter _openIaAdapter;
    private readonly IChatContextRepository _chatContextRepository;
    private readonly IUnitOfWork _unitOfWork;
    private readonly IMapper _mapper;
    private readonly IUserService _userService;
    private readonly IChannelService _channelService;

    private bool _successPermissions;
    private ChannelViewModel? _lastChannel;
    private UserViewModel? _lastUser;

    public IaChatService(
        IOpenIaAdapter openIaAdapter,
        IMapper mapper,
        IChatContextRepository chatContextRepository,
        IUnitOfWork unitOfWork, 
        IUserService userService, 
        IChannelService channelService)
    {
        _openIaAdapter = openIaAdapter;
        _mapper = mapper;
        _chatContextRepository = chatContextRepository;
        _unitOfWork = unitOfWork;
        _userService = userService;
        _channelService = channelService;

        _successPermissions = false;
        _lastChannel = null;
        _lastUser = null;
    }

    public async ValueTask<bool> CheckPermissions(ulong discordUserId, string userName, ulong discordChannelId)
    {
        var channel = await _channelService.GetByDiscordId(discordChannelId);
        _lastChannel = channel;
        _successPermissions = channel?.State.HasFlag(ChannelState.Enable) == true;

        if (!_successPermissions) return false;
        
        var userFilter = new UserRegisterViewModel(discordUserId, userName);
        var user = await _userService.AddOrGetUser(userFilter);
        _lastUser = user;
        _successPermissions = !_lastUser.State.HasFlag(UserState.Blocked); 
        
        return _successPermissions;
    }

    public async Task<string?> ProcessMessageSend(InputChatMessageViewModel input)
    {
        if (!_successPermissions || _lastUser is null || _lastChannel is null) return null;

        var newInput = input with
        {
            UserName = _lastUser.UserName
        };
        
        var chatInput = _mapper.Map<ChatMessageViewModel>(newInput);
        var context = await _chatContextRepository.GetChannelContext(input.DiscordChannelId);
        var contextToChat = _mapper.Map<LinkedList<ChatMessageViewModel>>(context);
        contextToChat.AddLast(chatInput);
        var userInputModel = _mapper.Map<ChatContextModel>(chatInput);
        userInputModel.ChannelId = input.DiscordChannelId;
        userInputModel.UserId = _lastUser.Id;
        await _chatContextRepository.Add(userInputModel);


        var response = await _openIaAdapter.SendChat(contextToChat);
        var responseModel = _mapper.Map<ChatContextModel>(response);
        responseModel.ChannelId = input.DiscordChannelId;
        await _chatContextRepository.Add(responseModel);

        await _unitOfWork.SaveChanges();

        return response.Message;
    }

    
}