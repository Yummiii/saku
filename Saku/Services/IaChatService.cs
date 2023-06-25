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

    public IaChatService(
        IOpenIaAdapter openIaAdapter,
        IMapper mapper,
        IChatContextRepository chatContextRepository,
        IUnitOfWork unitOfWork)
    {
        _openIaAdapter = openIaAdapter;
        _mapper = mapper;
        _chatContextRepository = chatContextRepository;
        _unitOfWork = unitOfWork;
    }

    public async Task<string> ProcessMessageSend(InputChatMessageViewModel input)
    {
        var newInput = input with
        {
            UserName = CleanUsername(input.UserName)
        };
        
        var chatInput = _mapper.Map<ChatMessageViewModel>(newInput);
        var context = await _chatContextRepository.GetChannelContext(input.DiscordChannelId);
        var contextToChat = _mapper.Map<LinkedList<ChatMessageViewModel>>(context);
        contextToChat.AddLast(chatInput);
        var userInputModel = _mapper.Map<ChatContextModel>(chatInput);
        userInputModel.ChannelId = input.DiscordChannelId;
        userInputModel.UserId = input.DiscordUserId;
        await _chatContextRepository.Add(userInputModel);


        var response = await _openIaAdapter.SendChat(contextToChat);
        var responseModel = _mapper.Map<ChatContextModel>(response);
        responseModel.ChannelId = input.DiscordChannelId;
        await _chatContextRepository.Add(responseModel);

        await _unitOfWork.SaveChanges();

        return response.Message;
    }

    private static string CleanUsername(string username)
    {
        var cleanUsername = Regex.Replace(username, @"[^a-zA-Z0-9_-]", string.Empty);
        if (cleanUsername.Length > 64)
        {
            cleanUsername = cleanUsername[..64];
        }

        return cleanUsername;
    }
}