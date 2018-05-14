//
//  init.h
//  vcx
//
//  Created by GuestUser on 4/30/18.
//  Copyright © 2018 GuestUser. All rights reserved.
//

#ifndef init_h
#define init_h
#import "libvcx.h"

extern void VcxWrapperCommonCallback(vcx_command_handle_t xcommand_handle,
                                     vcx_error_t err);

extern void VcxWrapperCommonHandleCallback(vcx_command_handle_t xcommand_handle,
                                           vcx_error_t err,
                                           vcx_command_handle_t pool_handle);

extern void VcxWrapperCommonStringCallback(vcx_command_handle_t xcommand_handle,
                                           vcx_error_t err,
                                           const char *const arg1);

extern void VcxWrapperCommonBoolCallback(vcx_command_handle_t xcommand_handle,
                                         vcx_error_t err,
                                         unsigned int arg1);

extern void VcxWrapperCommonStringStringCallback(vcx_command_handle_t xcommand_handle,
                                                 vcx_error_t err,
                                                 const char *const arg1,
                                                 const char *const arg2);

extern void VcxWrapperCommonStringOptStringCallback(vcx_command_handle_t xcommand_handle,
                                                    vcx_error_t err,
                                                    const char *const arg1,
                                                    const char *const arg2);

extern void VcxWrapperCommonDataCallback(vcx_command_handle_t xcommand_handle,
                                         vcx_error_t err,
                                         const uint8_t *const arg1,
                                         uint32_t arg2);

extern void VcxWrapperCommonStringStringStringCallback(vcx_command_handle_t xcommand_handle,
                                                       vcx_error_t err,
                                                       const char *const arg1,
                                                       const char *const arg2,
                                                       const char *const arg3);

extern void VcxWrapperCommonStringDataCallback(vcx_command_handle_t xcommand_handle,
                                               vcx_error_t err,
                                               const char *const arg1,
                                               const uint8_t *const arg2,
                                               uint32_t arg3);

extern void VcxWrapperCommonNumberCallback(vcx_command_handle_t xcommand_handle,
                                           vcx_error_t err,
                                           int32_t handle);

extern void VcxWrapperCommonStringOptStringOptStringCallback(vcx_command_handle_t xcommand_handle,
                                                             vcx_error_t err,
                                                             const char *const arg1,
                                                             const char *const arg2,
                                                             const char *const arg3);

void VcxWrapperCommonStringStringLongCallback(vcx_command_handle_t xcommand_handle,
                                              vcx_error_t err,
                                              const char *arg1,
                                              const char *arg2,
                                              unsigned long long arg3);


@interface ConnectMeVcx : NSObject

/**
 Calls when peer accepts a connection request from remoteDid.
 
 Alghoritm:
 1. Prepare pool and wallet if needed.
 2. Check if a pairwiseDid is already stored for remoteDid
 3. If pairwise is not stored:
     Create myDid with IndySignus:createAndStoreMyDid.
     Create pairwise pair remoteDid:myDid and store in wallet with IndyPairwise: createPairwise.
 4. Store metadata.
 5. Return completion block, containing error and json in format: {"verificationKey": "generatedVerificationKey", "userDID": "generated pairwise DID for passed remoteDID"}
 
 
 @param remoteDid Id of receiver identity.
 @param remoteVerkey Verification key of remoteDID.
 @param metadata Optional. Dictionaty with format: {String: Any}.
 @param completion Completion block, returns error and json with info about generated pairwise did. Will be invoked in Main thread.
 */
- (void)createOneTimeInfo:(NSString *)config
               completion:(void (^)(NSError *error, NSString *config))completion;

- (void)createConnectionWithInvite:(NSString *)invitationId
                     inviteDetails:(NSString *)inviteDetails
                        completion:(void (^)(NSError *error, NSString *credentialHandle))completion;

- (void)connectionHandle:(VcxHandle *)connectionHandle
          connectionType:(NSString *)connectionType
              completion:(void (^)(NSError *error, NSString *inviteDetails))completion;

- (void)updatePushToken:(NSString *)config
             completion:(void (^)(NSError *error, NSString *config))completion;

@end

#endif /* init_h */
