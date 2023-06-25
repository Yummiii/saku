using System.Text.RegularExpressions;
using AutoMapper;
using Lina.DynamicServicesProvider.Attributes;
using Saku.Models;
using Saku.Repositories.Interfaces;
using Saku.Services.Interfaces;
using Saku.ViewModels;

namespace Saku.Services;

[Service(typeof(IUserService))]
public class UserService : IUserService
{
    private readonly IUserRepository _userRepository;
    private readonly IUnitOfWork _unitOfWork;
    private readonly IMapper _mapper;

    public UserService(IUserRepository userRepository, IUnitOfWork unitOfWork, IMapper mapper)
    {
        _userRepository = userRepository;
        _unitOfWork = unitOfWork;
        _mapper = mapper;
    }

    public async Task<UserViewModel> AddOrGetUser(UserRegisterViewModel filter)
    {
        var userInDb = await _userRepository.GetByDiscordId(filter.DiscordId);
        if (userInDb is not null)
        {
            return _mapper.Map<UserViewModel>(userInDb);
        }

        var userModel = _mapper.Map<UserModel>(filter);
        await _userRepository.Add(userModel);
        await _unitOfWork.SaveChanges();
        return _mapper.Map<UserViewModel>(userModel);
    }

}