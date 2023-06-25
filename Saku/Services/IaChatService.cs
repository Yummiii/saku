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

[Service(typeof(IIaChatService), LifeTime.Singleton)]
public class IaChatService : IIaChatService
{
    private readonly IOpenIaAdapter _openIaAdapter;
    private readonly IChatContextRepository _chatContextRepository;
    private readonly IUnitOfWork _unitOfWork;
    private readonly IMapper _mapper;
    private readonly IUserService _userService;

    public IaChatService(
        IOpenIaAdapter openIaAdapter,
        IMapper mapper,
        IChatContextRepository chatContextRepository,
        IUnitOfWork unitOfWork, 
        IUserService userService)
    {
        _openIaAdapter = openIaAdapter;
        _mapper = mapper;
        _chatContextRepository = chatContextRepository;
        _unitOfWork = unitOfWork;
        _userService = userService;
    }

    public async Task<string> ProcessMessageSend(InputChatMessageViewModel input)
    {
        var userFilter = new UserRegisterViewModel(input.DiscordUserId, input.UserName);
        var user = await _userService.AddOrGetUser(userFilter);


        var newInput = input with
        {
            UserName = user.UserName
        };
        
        var chatInput = _mapper.Map<ChatMessageViewModel>(newInput);
        var context = await _chatContextRepository.GetChannelContext(input.DiscordChannelId);
        var contextToChat = _mapper.Map<LinkedList<ChatMessageViewModel>>(context);
        contextToChat.AddLast(chatInput);
        var userInputModel = _mapper.Map<ChatContextModel>(chatInput);
        userInputModel.ChannelId = input.DiscordChannelId;
        userInputModel.UserId = user.Id;
        await _chatContextRepository.Add(userInputModel);


        var response = await _openIaAdapter.SendChat(contextToChat);
        var responseModel = _mapper.Map<ChatContextModel>(response);
        responseModel.ChannelId = input.DiscordChannelId;
        await _chatContextRepository.Add(responseModel);

        await _unitOfWork.SaveChanges();

        return response.Message;
    }

    
}